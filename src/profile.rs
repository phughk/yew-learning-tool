use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div class="profile">
            <p>{"Profile page"}</p>
            <p>{"This page will include stats information"}</p>
            <p>{"Which datasets are most often accessed"}</p>
            <p>{"Learning curve"}</p>
            <p>{"Learning retention rate"}</p>
        </div>
    }
}
