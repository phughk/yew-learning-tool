use yew::prelude::*;
use crate::banner::Banner;
use crate::router::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Banner/>
            <Router/>
        </main>
    }
}
