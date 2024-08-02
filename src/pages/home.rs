use leptos::prelude::*;

use crate::components::Counter;

#[component]
pub fn MainRs() -> impl IntoView {
    let main_rs = std::fs::read_to_string("src/main.rs").unwrap_or_else(|err| {
        tracing::error!("Not found: {}", err);
        "Not Found".into()
    });

    view! {
        <pre class="text-xs">{ move || main_rs.clone() }</pre>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="block m-auto left-0 right-0 top-0 bottom-0 flex flex-col">
            <p class="text-red-700 text-5xl">"How to center a <div/>"</p>
            <Counter/>
        </div>
    }
}
