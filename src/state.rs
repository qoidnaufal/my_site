#[cfg(feature = "ssr")]
use {
    axum::extract::FromRef,
    leptos::prelude::LeptosOptions,
    leptos::{context::use_context, server_fn::ServerFnError},
    leptos_axum::AxumRouteListing,
    std::sync::Arc,
};

#[cfg(feature = "ssr")]
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
    pub counter: Arc<Counter>,
}

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct Counter {
    count: i32,
}

#[cfg(feature = "ssr")]
impl Counter {
    pub fn new(count: i32) -> Self {
        Self { count }
    }

    pub fn get(&self) -> i32 {
        self.count.clone()
    }

    pub fn inc(&mut self) {
        self.count += 1
    }

    pub fn dec(&mut self) {
        self.count -= 1
    }
}

#[cfg(feature = "ssr")]
pub async fn counter() -> Result<Counter, ServerFnError> {
    use_context::<Counter>().ok_or_else(|| ServerFnError::new("Unable to access counter"))
}
