use yew::prelude::*;
use crate::dataset::Dataset;

#[function_component(Datasets)]
pub fn datasets() -> Html {
    let items = vec![
        "one",
        "two",
        "three"
    ];
    html! {
        <div class="datasets">
            <p>{"Datasets page"}</p>
            <ul class="item-list">
                { items.iter().map(|props| {
                    html!{<Dataset/>}
                }).collect::<Html>() }
            </ul>
        </div>
    }
}
