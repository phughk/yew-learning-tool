use yew_router::prelude::*;
use yew::prelude::*;
use crate::about::About;
use crate::profile::Profile;
use crate::datasets::Datasets;
use crate::repository::Repository;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/dataset")]
    Dataset,
    #[at("/repository")]
    Repository,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <About/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Profile => html! { <Profile/>},
        Route::Dataset => html! {<Datasets/>},
        Route::Repository => html! {<Repository/>},
        Route::About => html! {<About/>}
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
