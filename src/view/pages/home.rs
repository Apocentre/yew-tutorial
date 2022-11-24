use yew::prelude::*;
use yewdux::prelude::*;
use crate::data::{
  msg::{AsyncMsg, AsyncResult}, state::State,
};

#[function_component(Home)]
pub fn home() -> Html {
  let (state, dispatch) = use_store::<State>();
  let async_action_example = dispatch.apply_callback(|_| AsyncMsg::FetchCats);

  let cats = match state.cats {
    AsyncResult::Loading => html!(<div>{"Loading"}</div>),
    AsyncResult::Success(ref _cats) => html!(<div>{"Loaded Cats"}</div>),
    AsyncResult::Failed(ref error) => html!(<div>{format!("Error happened {}", error)}</div>),
    _ => html!(),
  };

  html!(
    <div>
      <h1>{"Home Page"}</h1>
      <button onclick={async_action_example}>{"Async Call"}</button>
      {cats}
    </div>
  )
}
