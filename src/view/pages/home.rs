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
      <button onclick={|_| show_congrats_toast("This works")}>{"Interop With Toastify"}</button>
      <button
        onclick={async_action_example}
        class={classes!("btn")}
      >
        {"Fetch Cats"}
      </button>
      {cats}
    </div>
  )
}
