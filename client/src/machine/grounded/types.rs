use crate::machine::common::*;

pub enum State {
    Hopping,
    Crouching,
    Crawling,
    Walking,
    Dashing,
    Landing,
    CharacterAction,
}

pub trait Characteristics {
    fn countdown(&self, state: &State) -> i8;
}
