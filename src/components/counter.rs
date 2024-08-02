use leptos::ev;
use leptos::island;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

use crate::api::{dec_counter, get_counter, inc_counter};

#[island]
pub fn Counter() -> impl IntoView {
    let inc = Action::new(|_: &()| inc_counter());
    let dec = Action::new(|_: &()| dec_counter());

    let counter = Resource::new(
        move || (inc.version().get(), dec.version().get()),
        move |_| get_counter(),
    );

    let inc = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        inc.dispatch(());
    };
    let dec = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        dec.dispatch(());
    };

    let button_class = "bg-green-300 w-[100px] py-2 rounded-lg";

    view! {
        <div class="items-center flex flex-col">
            <Suspense>
                <p class="text-red-500">
                    { move || counter }
                </p>
            </Suspense>
            <div class="flex flex-row justify-center space-x-2 w-[300px]">
                <button
                    class=button_class
                    on:click=dec
                >
                    "-"
                </button>
                <button
                    class=button_class
                    on:click=inc
                >
                    "+"
                </button>
            </div>
        </div>
    }
}
