use yew::prelude::*;

#[function_component(Dataset)]
pub fn dataset() -> Html {
    html! {
        <div class={classes!("dataset", "bg-red-100")}>
            <p class={classes!("bg-red-100")}>{"Test!"}</p>
            <p>{"This is a single dataset"}</p>
        </div>
    }
}
