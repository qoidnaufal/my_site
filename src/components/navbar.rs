use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="flex flex-row bg-slate-200">
            <button class="py-2 px-2 hover:bg-green-300">"Home"</button>
            <button class="py-2 px-2 hover:bg-green-300">"Dashboard"</button>
            <button class="py-2 px-2 hover:bg-green-300">"Contact Me"</button>
        </div>
    }
}
