use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
  pub navbar_active: bool,
}
