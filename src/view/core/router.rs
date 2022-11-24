use yew::prelude::*;
use yew_router::prelude::*;
use crate::view::pages::{
  home::Home, profile::Profile,
};

#[derive(Routable, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Route {
  #[at("/")]
  Home,

  #[at("/profile")]
  Profile,

  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Home /> },
    Route::Profile => html! { <Profile /> },
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}
