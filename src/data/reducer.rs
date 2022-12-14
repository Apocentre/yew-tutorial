use std::rc::Rc;
use yewdux::prelude::*;
use super::{
  state::State, msg::Msg,
};

impl Reducer<State> for Msg {
  fn apply(&self, mut state: Rc<State>) -> Rc<State> {
    let mut_state = Rc::make_mut(&mut state);

    match self {
      Msg::ToggleNavbar => {
        mut_state.navbar_active = !mut_state.navbar_active
      },
      Msg::SetCatsResult(cats) => {
        mut_state.cats = cats.clone();
      },
    };

    state
  }
}
