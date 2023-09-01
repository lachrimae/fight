use crate::input::Button;
use crate::machine::types::{Armour, Physics};
use crate::world::{ButtonDiff, InputDiff};
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Hash)]
pub enum State {
    #[default]
    Standing,
    Jabbing,
}

fn timeout_state(state: State) -> State {
    use self::State::*;
    match state {
        Standing => Standing,
        Jabbing => Jabbing,
    }
}

fn timeout(state: State) -> i8 {
    use self::State as S;
    match state {
        S::Standing => -1,
        S::Jabbing => 13,
    }
}

struct FrameData {
    physics: Physics,
    armour: Armour,
}

fn state_frame_data(state: State, frame: u8) -> FrameData {
    unimplemented!()
}

fn standing_input_map(input: InputDiff) -> Option<State> {
    if input.get(Button::Hit) == ButtonDiff::Pressed {
        Some(State::Jabbing)
    } else {
        None
    }
}

fn user_input_map(state: State, input: InputDiff) -> Option<State> {
    use self::State as S;
    match state {
        S::Standing => standing_input_map(input),
        S::Jabbing => None,
    }
}

pub fn postbox_input_system(mut query: Query<&mut State, &InputDiff>) {
    for mut postbox_state in query.iter_mut() {
        unimplemented!()
    }
}
