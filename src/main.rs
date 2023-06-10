mod app;
mod banner;
mod profile;
mod about;
mod datasets;
mod repository;
mod router;
mod not_found;
mod dataset;

use app::App;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
