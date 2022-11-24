use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::data::{
  msg::Msg, state::State,
};

use super::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
  let (state, dispatch) = use_store::<State>();
  let toggler_navbar = dispatch.apply_callback(|_| Msg::ToggleNavbar);

  let view_nav = || {
    let active_class = if !state.navbar_active {"is-active"} else {""};

    html!(
      <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
        <div class="navbar-brand">
          <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

          <button class={classes!("navbar-burger", "burger", active_class)}
            aria-label="menu" aria-expanded="false"
            onclick={toggler_navbar}
          >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </button>
        </div>
        <div class={classes!("navbar-menu", active_class)}>
          <div class="navbar-start">
            <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
              { "Home" }
            </Link<Route>>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Profile}>
              { "Profile" }
            </Link<Route>>
          </div>
        </div>
      </nav>
    )
  };

  html!(
    <BrowserRouter>
      {view_nav()}
      <main>
        <Switch<Route> render={Switch::render(switch)} />
      </main>
      <footer class="footer">
        <div class="content has-text-centered">
          { "Powered by " }
          <a href="https://yew.rs">{ "Yew" }</a>
          { " using " }
          <a href="https://bulma.io">{ "Bulma" }</a>
          { " and images from " }
          <a href="https://unsplash.com">{ "Unsplash" }</a>
        </div>
      </footer>
    </BrowserRouter>
  )
}
