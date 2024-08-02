use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NavBar() -> impl IntoView {
    let nav_class = "py-2 px-2 hover:bg-green-300";

    view! {
        <div class="flex flex-row bg-slate-200">
            <Home nav_class/>
            <MainRs nav_class/>
            <div class=nav_class>"Contact Me"</div>
        </div>
    }
}

#[component]
fn Home(nav_class: &'static str) -> impl IntoView {
    view! {
        <A href="/">
            <div class=nav_class>"Home"</div>
        </A>
    }
}

#[component]
fn MainRs(nav_class: &'static str) -> impl IntoView {
    view! {
        <A href="/main_rs">
            <div class=nav_class>"Main File"</div>
        </A>
    }
}
