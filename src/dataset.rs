use yew::prelude::*;

use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub tags: Vec<AttrValue>,
    #[prop_or_default]
    pub description: AttrValue,
    #[prop_or_default]
    pub author: AttrValue,
    #[prop_or_default]
    pub last_used: AttrValue,
    #[prop_or_default]
    pub progress: AttrValue,
}

#[function_component(Dataset)]
pub fn dataset(props: &Props) -> Html {
    let shadow = match props.name.as_str() {
        "two" => "shadow-lg",
        _ => "shadow-sm"
    };
    html! {
        <div class={classes!("bg-white", "rounded-lg", {{shadow}}, "p-4")}>
          <h2 class={classes!("text-2xl", "font-bold", "text-accent", "mb-2")}>{&props.name}</h2>
          <p class={classes!("text-gray-600", "mb-4")}>{"Tags: "}{for props.tags.iter()}</p>
          <p class={classes!("text-gray-600", "mb-4")}>{"Description: "}{&props.description}</p>
          <div class={classes!("flex", "items-center", "justify-between")}>
            <div class={classes!("text-gray-500")}>{"Author: "}{&props.author}</div>
            <div class={classes!("text-right")}>
              <span class={classes!("text-gray-500")}>{"Last Used: "}{&props.last_used}</span>
              <span class={classes!("ml-2", "text-gray-500")}>{"Progress: "}{&props.progress}</span>
            </div>
          </div>
        </div>

}
}
