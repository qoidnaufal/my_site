mod app;
mod components;
mod fileserv;
mod pages;

pub use app::App;
#[cfg(feature = "ssr")]
pub use fileserv::file_and_error_handler;

#[cfg(feature = "ssr")]
use {
    axum::{
        body::Body as AxumBody,
        extract::{FromRef, State},
        http::Request,
        response::{IntoResponse, Response},
    },
    leptos::LeptosOptions,
    leptos_axum::handle_server_fns_with_context,
    leptos_router::RouteListing,
};

#[cfg(feature = "ssr")]
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
}

#[cfg(feature = "ssr")]
pub async fn server_fn_handler(request: Request<AxumBody>) -> impl IntoResponse {
    handle_server_fns_with_context(move || (), request).await
}

#[cfg(feature = "ssr")]
pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    request: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || (),
        App,
    );
    handler(request).await.into_response()
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
