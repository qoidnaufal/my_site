mod api;
mod app;
mod components;
mod fileserv;
mod pages;
mod state;

pub use app::App;
use leptos::prelude::*;
use leptos_meta::MetaTags;

#[cfg(feature = "ssr")]
pub use {
    fileserv::file_and_error_handler,
    state::{AppState, Counter},
};

#[cfg(feature = "ssr")]
use {
    axum::{
        body::Body as AxumBody,
        extract::State,
        http::Request,
        response::{IntoResponse /* , Response */},
    },
    leptos::prelude::LeptosOptions,
    leptos_axum::handle_server_fns_with_context,
};

#[cfg(feature = "ssr")]
pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    use std::sync::Arc;

    let counter = Arc::clone(&app_state.counter);

    handle_server_fns_with_context(
        move || {
            provide_context(counter.clone());
        },
        request,
    )
    .await
}

// #[cfg(feature = "ssr")]
// pub async fn leptos_routes_handler(
//     State(app_state): State<AppState>,
//     request: Request<AxumBody>,
// ) -> Response {
//     let handler = leptos_axum::render_route_with_context(
//         app_state.routes.clone(),
//         move || {
//             provide_context(app_state.counter);
//         },
//         App,
//     );
//     handler(request).await.into_response()
// }

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
