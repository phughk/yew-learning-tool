use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="banner">
            <a href="/profile">{"Profile"}</a>
            <a href="/datasets">{"Datasets"}</a>
            <a href="/about">{"About"}</a>
            <a href="/repository">{"Repository"}</a>
        </div>
    }
}
