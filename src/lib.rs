mod app;
use app::Model;
use wasm_bindgen_futures::wasm_bindgen;
mod resouces;
mod judge;
mod builtin_words;
use crate::wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}