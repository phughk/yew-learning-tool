use yew::prelude::*;

#[function_component(Dataset)]
pub fn dataset() -> Html {
    html! {
        <div class={classes!("bg-white", "rounded-lg", "shadow-md", "p-4")}>
  <h2 class={classes!("text-2xl", "font-bold", "text-accent", "mb-2")}>{"Dataset Name"}</h2>
  <p class={classes!("text-gray-600", "mb-4")}>{"Tags: Tag1, Tag2, Tag3"}</p>
  <p class={classes!("text-gray-600", "mb-4")}>{"Description: Lorem ipsum dolor sit amet, consectetur adipiscing elit."}</p>
  <div class={classes!("flex", "items-center", "justify-between")}>
    <div class={classes!("text-gray-500")}>{"Author: John Doe"}</div>
    <div class={classes!("text-right")}>
      <span class={classes!("text-gray-500")}>{"Last Used: 2023-05-13 09:30 AM"}</span>
      <span class={classes!("ml-2", "text-gray-500")}>{"Progress: 80%"}</span>
    </div>
  </div>
</div>

}
}
