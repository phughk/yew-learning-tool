use yew::prelude::*;

#[function_component(Repository)]
pub fn repository() -> Html {
    html! {
        <div class="repository">
            <p>{"Repository page"}</p>
            <p>{"The purpose of the repository page is to point to servers that host learning files."}</p>
            <p>{"These files can then be downloaded to the local host."}</p>
        </div>
    }
}
