use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="not-found">
            <p>{"Not Found page"}</p>
        </div>
    }
}
