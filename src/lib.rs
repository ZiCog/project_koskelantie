#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
//use yew::web_sys;
//use yew::services::ConsoleService;

mod app;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    if let Some(elem) = yew::utils::document().query_selector("#app").unwrap() {
        App::<app::App>::new().mount(elem);
        Ok(())
    } else {
        Err(JsValue::from(format!("No such element {}", "#app")))
    }
}
