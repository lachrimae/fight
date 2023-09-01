use crate::input::Button;
use crate::machine::types::{Armour, Physics};
use crate::world::{ButtonDiff, InputDiff, Orientation, StandingOn};
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default, Reflect, PartialEq, Eq)]
pub enum GroundedStance {
    #[default]
    Standing,
    Jabbing,
}

#[derive(Copy, Clone, Debug, Default, Reflect, PartialEq, Eq)]
pub enum AerialStance {
    #[default]
    Falling,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum Stance {
    Grounded(GroundedStance),
    Aerial(AerialStance),
}

impl Default for Stance {
    fn default() -> Self {
        Stance::Grounded(GroundedStance::default())
    }
}

#[derive(Component, Default, Reflect)]
pub struct PostboxState {
    stance: Stance,
    orientation: Orientation,
    countdown: i8,
    countup: u8,
}

fn timeout_stance(state: Stance) -> Stance {
    use self::AerialStance as A;
    use self::GroundedStance as G;
    use self::Stance as S;
    match state {
        S::Aerial(_) => S::Aerial(A::Falling),
        S::Grounded(_) => S::Grounded(G::Standing),
    }
}

fn timeout(state: Stance) -> i8 {
    use self::AerialStance as A;
    use self::GroundedStance as G;
    use self::Stance as S;
    match state {
        S::Grounded(G::Standing) => -1,
        S::Grounded(G::Jabbing) => 13,
        S::Aerial(A::Falling) => -1,
    }
}

struct FrameData {
    physics: Physics,
    armour: Armour,
}

fn state_frame_data(state: Stance, frame: u8) -> FrameData {
    unimplemented!()
}

fn standing_input_map(input: InputDiff) -> Option<GroundedStance> {
    use self::GroundedStance as G;
    if input.get(Button::Hit) == ButtonDiff::Pressed {
        Some(G::Jabbing)
    } else {
        None
    }
}

fn falling_input_map(input: InputDiff) -> Option<AerialStance> {
    None
}

fn grounded_user_input_map(state: GroundedStance, input: InputDiff) -> Option<GroundedStance> {
    use self::GroundedStance as G;
    match state {
        G::Standing => standing_input_map(input),
        G::Jabbing => None,
    }
}

fn aerial_user_input_map(state: AerialStance, input: InputDiff) -> Option<AerialStance> {
    use self::AerialStance as A;
    match state {
        A::Falling => falling_input_map(input),
    }
}

fn update_stance(state: &mut PostboxState, new_stance: Stance) {
    state.countup = 0;
    state.countdown = timeout(new_stance);
    state.stance = new_stance;
}

pub fn postbox_input_system(
    mut query: Query<(&mut PostboxState, &InputDiff, Option<&StandingOn>)>,
) {
    use self::Stance as S;
    for (mut state, input, standing_on) in query.iter_mut() {
        if let Some(new_stance) = match state.stance {
            S::Grounded(g) => grounded_user_input_map(g, *input).map(|res| S::Grounded(res)),
            S::Aerial(a) => aerial_user_input_map(a, *input).map(|res| S::Aerial(res)),
        } {
            update_stance(&mut state, new_stance);
        } else {
            state.countup += 1;
            if state.countdown > 0 {
                state.countdown -= 1;
            }
        }
        if state.countdown == 0 {
            let new_stance = timeout_stance(state.stance);
            if new_stance != state.stance {
                update_stance(&mut state, new_stance);
            }
        }
    }
}
