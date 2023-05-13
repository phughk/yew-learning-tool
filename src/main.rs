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

fn main() {
    yew::Renderer::<App>::new().render();
}
