use crate::machine::common::*;

pub enum GroundAction {
    Hopping,
    Walking,
    Dashing,
    CharacterAction,
}

pub struct GroundedState {
    action: GroundAction,
}

impl Machine for GroundedState {
    fn take_input(&self)
}
