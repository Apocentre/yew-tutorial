use crate::data::model::cat::Cat;

#[derive(Clone, PartialEq, Eq)]
pub enum AsyncResult<T> {
  NotStarted,
  Loading,
  Success(T),
  Failed(String),
}

pub enum Msg {
  ToggleNavbar,
  SetCatsResult(AsyncResult<Vec<Cat>>),
}

pub enum AsyncMsg {
  FetchCats,
}
