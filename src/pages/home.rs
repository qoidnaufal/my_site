use crate::components::NavBar;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <NavBar/>
        <div class="block m-auto left-0 right-0 top-0 bottom-0 flex flex-col">
            <p class="text-red-700 text-5xl">"How to center a <div/>"</p>
        </div>
    }
}
