use yew::prelude::*;
use yew::props;
use yew::virtual_dom::VNode;
use crate::dataset::{Dataset, Focused, Props};

#[function_component(Datasets)]
pub fn datasets() -> Html {
    let props = vec![
        props!(Props{name: "First", tags: vec!["one".into(), "two".into()], focused: Focused::NotFocused}),
        props!(Props{name: "Second", tags: vec!["three".into(), "four".into()], focused: Focused::Clicked}),
        props!(Props{name: "Third", tags: vec!["five".into()], focused: Focused::Hover}),
    ];
    html! {
        <div class="datasets">
            <p>{"Datasets page"}</p>
            <ul>
                {props.into_iter().map(|props| {
                    html!{ <li><Dataset ..props /></li>}
                }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
