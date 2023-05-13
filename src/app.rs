use yew_router::prelude::*;
use yew::prelude::*;
use crate::banner::Banner;
use crate::router::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Banner/>
                <Router/>
            </BrowserRouter>
        </main>
    }
}
