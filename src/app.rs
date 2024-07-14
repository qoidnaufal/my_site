use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::NavBar;
use crate::pages::{Home, MainRs};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="/pkg/my_site.css"/>
        <Title text="My Site"/>
        <Link rel="shortcut icon" type_="image/ico" href="/icon.ico"/>
        <Meta name="description" content="My site built with Leptos"/>
        <Router>
            <main class="absolute m-auto top-0 left-0 bottom-0 right-0 size-full flex flex-col">
                <NavBar/>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/main_rs" view=MainRs/>
                </Routes>
            </main>
        </Router>
    }
}
