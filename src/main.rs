mod app;
mod banner;
mod profile;
mod about;
mod datasets;
mod repository;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
