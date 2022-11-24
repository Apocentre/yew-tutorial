use yew::prelude::*;
use yewdux::prelude::*;
use crate::{
  data::{
    msg::{AsyncMsg, AsyncResult}, state::State,
  },
  view::interop::{
    use_toast, toast::{show_congrats_toast},
  }
};

#[function_component(Home)]
pub fn home() -> Html {
  let (state, dispatch) = use_store::<State>();
  let async_action_example = dispatch.apply_callback(|_| AsyncMsg::FetchCats);

  let toast_ready = use_toast();

  let cats = match state.cats {
    AsyncResult::Loading => html!(<div>{"Loading"}</div>),
    AsyncResult::Success(ref _cats) => html!(<div>{"Loaded Cats"}</div>),
    AsyncResult::Failed(ref error) => html!(<div>{format!("Error happened {}", error)}</div>),
    _ => html!(),
  };

  log::info!("toast_ready: {:?}", toast_ready);

  html!(
    <div class="flex space-x-2 justify-center">
      <h1>{"Home Page"}</h1>
      <button onclick={|_| show_congrats_toast("This works")}>{"Interop With Toastify"}</button>
      <button
        onclick={async_action_example}
        type="button"
        data-mdb-ripple="true"
        data-mdb-ripple-color="light"
        class="inline-block px-6 py-2.5 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
      >
        {"Fetch Cats"}
      </button>
      {cats}
    </div>
  )
}
