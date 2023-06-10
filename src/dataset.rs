use yew::prelude::*;

use yew::{function_component, html, Html, Properties, events};
use log::info;
use std::format;

#[derive(PartialEq, Default)]
pub enum Focused {
    #[default]
    NotFocused,
    Hover,
    Clicked,
}

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
    #[prop_or_default]
    pub focused: Focused,
    #[prop_or(None)]
    pub on_hover: Option<Callback<bool>>,
    #[prop_or(None)]
    pub on_click: Option<Callback<bool>>,
}

#[function_component(Dataset)]
pub fn dataset(props: &Props) -> Html {
    // Listeners
    let event_listener: Callback<events::MouseEvent> = Callback::from(|e: events::MouseEvent| {
        // do stuff
        let s = format!("Testing - {:?}", e);
        info!("{}", s)
    });
    // Conditionals
    let shadow = match &props.focused {
        Focused::Clicked => "shadow-lg",
        Focused::Hover => "shadow-m",
        _ => "shadow-sm"
    };
    html! {
        <div class={classes!("bg-white", "rounded-lg", {{shadow}}, "p-4") } onclick={event_listener.clone()} onmouseover={event_listener}>
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
