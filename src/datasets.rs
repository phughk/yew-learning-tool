use yew::prelude::*;
use yew::props;
use yew::virtual_dom::VNode;
use crate::dataset::{Dataset, Props};

#[function_component(Datasets)]
pub fn datasets() -> Html {
    let props = vec![
        props!(Props{name: "First", tags: vec!["one".into(), "two".into()]}),
        props!(Props{name: "Second", tags: vec!["three".into(), "four".into()]}),
        props!(Props{name: "Third", tags: vec!["five".into()]}),
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
