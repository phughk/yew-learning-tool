use yew::prelude::*;
use crate::banner::Banner;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Banner/>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
