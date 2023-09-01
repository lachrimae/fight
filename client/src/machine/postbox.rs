use crate::input::Button;
use crate::machine::types::{Armour, Physics};
use crate::world::{
    Acceleration, ButtonDiff, InputDiff, Orientation, Position, StandingOn, Velocity,
};
use bevy::log;
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

#[derive(Component, Reflect)]
pub struct PostboxState {
    pub stance: Stance,
    pub orientation: Orientation,
    countdown: i8,
    countup: u8,
}

impl Default for PostboxState {
    fn default() -> Self {
        PostboxState {
            stance: Stance::default(),
            orientation: Orientation::default(),
            countdown: -1,
            countup: 0,
        }
    }
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

fn stance_frame_data(stance: Stance, frame: u8) -> FrameData {
    use self::AerialStance as A;
    use self::GroundedStance as G;
    use self::Stance as S;
    use Armour as R;
    use Physics as P;
    match stance {
        S::Grounded(G::Standing) => FrameData {
            physics: P::NotMoving,
            armour: R::None,
        },
        S::Grounded(G::Jabbing) => FrameData {
            physics: P::NotMoving,
            armour: R::None,
        },
        S::Aerial(A::Falling) => FrameData {
            physics: P::Falling,
            armour: R::None,
        },
    }
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

fn grounded_user_input_map(
    state: GroundedStance,
    _frame: u8,
    input: InputDiff,
) -> Option<GroundedStance> {
    use self::GroundedStance as G;
    match state {
        G::Standing => standing_input_map(input),
        G::Jabbing => None,
    }
}

fn aerial_user_input_map(
    state: AerialStance,
    _frame: u8,
    input: InputDiff,
) -> Option<AerialStance> {
    use self::AerialStance as A;
    match state {
        A::Falling => falling_input_map(input),
    }
}

fn update_stance(state: &mut PostboxState, new_stance: Stance) {
    state.countup = 0;
    log::trace!("Executing frame 1 of {:?}", new_stance);
    state.countdown = timeout(new_stance);
    state.stance = new_stance;
    assert!(state.countdown != 0);
}

fn tick_stance(state: &mut PostboxState) {
    state.countup = state.countup.wrapping_add(1);
    if state.countdown > 0 {
        state.countdown -= 1;
    }
    assert!(state.countdown >= -1);
    if state.countdown == 0 {
        log::trace!("Stance timeout");
        let new_stance = timeout_stance(state.stance);
        update_stance(state, new_stance);
    } else {
        log::trace!(
            "Executing frame {:?} of {:?}",
            state.countup + 1,
            state.stance
        );
    }
}

pub fn input_system(mut query: Query<(&mut PostboxState, &mut Physics, &mut Armour, &InputDiff)>) {
    use self::Stance as S;
    log::debug!("postbox input system beginning");
    for (mut state, mut physics, mut armour, input) in query.iter_mut() {
        let frame = state.countup;
        if let Some(new_stance) = match state.stance {
            S::Grounded(g) => grounded_user_input_map(g, frame, *input).map(|res| S::Grounded(res)),
            S::Aerial(a) => aerial_user_input_map(a, frame, *input).map(|res| S::Aerial(res)),
        } {
            log::trace!("Setting stance to {new_stance:?}");
            update_stance(&mut state, new_stance);
        } else {
            tick_stance(&mut state);
        }
        let frame_data = stance_frame_data(state.stance, state.countup);
        *armour = frame_data.armour;
        *physics = frame_data.physics;
    }
}

pub fn physics_system(
    mut query: Query<(&Physics, &mut Position, &mut Velocity, &mut Acceleration)>,
) {
    log::debug!("postbox physics system beginning");
    for (physics, mut pos, mut vel, mut acc) in query.iter_mut() {
        match physics {
            Physics::NotMoving => {
                vel.x = 0;
                vel.y = 0;
                acc.x = 0;
                acc.y = 0;
            }
            Physics::Falling => vel.y = -10,
        }
    }
}
