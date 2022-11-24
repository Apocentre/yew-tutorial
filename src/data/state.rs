use yewdux::prelude::*;
use crate::data::{
  model::cat::Cat,
  msg::AsyncResult,
};


#[derive(Clone, PartialEq, Eq, Store)]
pub struct State {
  pub navbar_active: bool,
  pub cats: AsyncResult<Vec<Cat>>,
}

impl Default for State {
    fn default() -> Self {
      Self {
        navbar_active: Default::default(),
        cats: AsyncResult::NotStarted
      }
    }
}
