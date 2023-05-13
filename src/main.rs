mod app;
mod banner;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
