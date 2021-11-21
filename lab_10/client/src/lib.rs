use wasm_bindgen::prelude::*;
mod switch;
mod pages;
mod app;
use app::Index;
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Index>();
    Ok(())
}