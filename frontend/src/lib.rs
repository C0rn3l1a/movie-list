pub extern crate common;

// bring libs into scope
use wasm_bindgen::prelude::*;
use yew::prelude::*;

// define components module
mod components;

// Bring ListView into scope
use components::list_view::ListView;

#[function_component(Base)]
fn app() -> Html {
    html! {
        <ListView></ListView>
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Base>();
}
