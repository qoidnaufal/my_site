use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

use crate::components::NavBar;
use crate::pages::{Home, MainRs};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="/pkg/my_site.css"/>
        <Title text="My Site"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="My site built with Leptos"/>
        <Router>
            <main class="absolute m-auto top-0 left-0 bottom-0 right-0 size-full flex flex-col">
                <NavBar/>
                <FlatRoutes fallback=|| "Not Found...">
                    <Route path=StaticSegment("/") view=Home/>
                    <Route path=StaticSegment("/main_rs") view=MainRs/>
                </FlatRoutes>
            </main>
        </Router>
    }
}
