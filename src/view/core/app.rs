use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::{
  data::{
    msg::Msg, state::State,
  },
  view::interop::ResourceProvider,
};

use super::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
  let view_nav = || {
    html!(
      <nav class="navbar bg-base-100" role="navigation" aria-label="main navigation">
        <div class="navbar-start">
        <div class="dropdown">
          <label  tabindex="0" class="btn btn-ghost btn-circle">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg>
          </label>
          <ul tabindex="0" class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
            <li>
              <Link<Route> to={Route::Home}>
                { "Home" }
              </Link<Route>>
            </li>
            <li>
              <Link<Route> to={Route::Profile}>
                { "Profile" }
              </Link<Route>>
            </li>
          </ul>
        </div>
        </div>
        <div class="navbar-center">
          <Link<Route> classes={classes!("btn", "btn-ghost", "normal-case", "text-xl")} to={Route::Home}>
            { "Home" }
          </Link<Route>>
          <Link<Route> classes={classes!("btn", "btn-ghost", "normal-case", "text-xl")} to={Route::Profile}>
            { "Profile" }
          </Link<Route>>
        </div>
      </nav>
    )
  };

  html!(
    <ResourceProvider>
      <BrowserRouter>
        <div class="min-h-screen">
          {view_nav()}
          <main>
            <Switch<Route> render={Switch::render(switch)} />
          </main>
          <footer class="footer footer-center p-4 bg-base-300 text-base-content sticky top-[100vh]">
            <div>
              <p>{"Copyright © 2022 - All right reserved by Apocentre"}</p>
            </div>
          </footer>
        </div>
      </BrowserRouter>
    </ResourceProvider>
  )
}
