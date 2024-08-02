#[cfg(feature = "ssr")]
use {
    axum::{routing::get, Router},
    leptos::config::get_configuration,
    leptos::context::provide_context,
    leptos_axum::{generate_route_list, LeptosRoutes},
    std::sync::Arc,
    tokio::net::TcpListener,
    tower_http::compression::{
        predicate::{NotForContentType, SizeAbove},
        CompressionLayer, CompressionLevel, Predicate,
    },
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

#[cfg(feature = "ssr")]
use {
    my_site::file_and_error_handler,
    my_site::server_fn_handler,
    my_site::{shell, App},
    my_site::{AppState, Counter},
};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "my_site=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conf = get_configuration(Some("Cargo.toml"))?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let app_routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        routes: app_routes.clone(),
        counter: Arc::new(Counter::new(0)),
    };

    let predicate = SizeAbove::new(1500)
        .and(NotForContentType::GRPC)
        .and(NotForContentType::IMAGES)
        .and(NotForContentType::const_new("application/javascript"))
        .and(NotForContentType::const_new("application/wasm"))
        .and(NotForContentType::const_new("text/css"));

    let router = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_context(
            &app_state,
            app_routes,
            {
                let counter = Arc::clone(&app_state.counter);
                move || {
                    provide_context(counter.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Fastest)
                .compress_when(predicate),
        )
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
