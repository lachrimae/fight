use bevy::prelude::*;

type UserInput = ();

#[derive(Component, Default, Reflect, Hash)]
pub enum State {
    #[default]
    Standing,
    Punching,
}

pub enum Physics {
    NotMoving,
}

pub enum Armour {
    None,
    HyperArmour,
    Invincibility,
}

fn timeout_state(state: State) -> State {
    use self::State::*;
    match state {
        Standing => Standing,
        Punching => Punching,
    }
}

static standing_frame_data: [FrameData; 1] = [FrameData {
    physics: Physics::NotMoving,
    armour: Armour::None,
}];

fn timeout(state: State) -> i8 {
    use self::State::*;
    match state {
        Standing => -1,
        Punching => 13,
    }
}

struct FrameData {
    physics: Physics,
    armour: Armour,
}

fn state_frame_data(state: State, frame: u8) -> FrameData {
    unimplemented!()
}

fn user_input_map(state: State, input: UserInput) -> State {
    unimplemented!()
}

pub fn postbox_input_system(mut query: Query<&mut State>) {
    for mut postbox_state in query.iter_mut() {
        unimplemented!()
    }
}
