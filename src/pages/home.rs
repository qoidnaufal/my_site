use leptos::*;

#[island]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0);
    let inc = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        count.update(|n| *n += 1);
    };
    let dec = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        count.update(|n| *n -= 1);
    };
    let button_class = "bg-green-300 w-[100px] py-2 rounded-lg";

    view! {
        <div class="items-center flex flex-col">
            <p class="text-red-500">{ move || count.get() }</p>
            <div class="flex flex-row justify-center space-x-2 w-[300px]">
                <button
                    class=button_class
                    on:click=inc
                >
                    "+"
                </button>
                <button
                    class=button_class
                    on:click=dec
                >
                    "-"
                </button>
            </div>
        </div>
    }
}

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
