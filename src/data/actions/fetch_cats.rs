use std::rc::Rc;
use eyre::Result;
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;
use gloo_net::http::Request;
use gloo_timers::future::sleep;
use crate::data::{
  msg::{Msg, AsyncMsg, AsyncResult}, model::cat::Cat,
  state::State,
};

async fn fetch_cats() -> Result<Vec<Cat>> {
  let res = Request::get("https://api.thecatapi.com/v1/images/search")
  .send()
  .await?
  .json::<Vec<Cat>>()
  .await?
  .into_iter()
  .collect::<Vec<_>>();

  // add delay to show the state changes i.e from Loading to Success
  sleep(core::time::Duration::from_secs(2)).await;

  Ok(res)
}

impl Reducer<State> for AsyncMsg {
  fn apply(&self, mut state: Rc<State>) -> Rc<State> {
    let mut_state = Rc::make_mut(&mut state);

    spawn_local(async move {
      let cats = match fetch_cats().await {
        Ok(cats) => AsyncResult::Success(cats),
        Err(error) => AsyncResult::Failed(error.to_string()),
      };
  
      Dispatch::<State>::new().apply(Msg::SetCatsResult(cats));
    });

    mut_state.cats = AsyncResult::Loading;

    state
  }
}
