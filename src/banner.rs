use yew_router::prelude::*;
use yew::prelude::*;
use crate::router::Route;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="banner">
            <Link<Route> to={Route::Profile}>{ "Profile" }</Link<Route>>
            <Link<Route> to={Route::Dataset}>{ "Datasets" }</Link<Route>>
            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
            <Link<Route> to={Route::Repository}>{ "Repository" }</Link<Route>>
        </div>
    }
}
