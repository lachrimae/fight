use crate::world::*;
use bevy::log;
use bevy::prelude::*;
use std::option::Option;

fn num_countdown_frames(action: Action) -> i8 {
    match action {
        Action::Standing => -1,
        Action::Falling(_) => -1,
        Action::Walking => -1,
        Action::Jabbing => 13,
        Action::NAiring(_) => 40,
    }
}

fn modify_neutral(last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Standing,
        Action::Falling(n) => Action::Falling(n),
        Action::Walking => Action::Standing,
        Action::Jabbing => Action::Standing,
        Action::NAiring(n) => Action::Falling(n),
    };
    (last_o, a)
}

fn modify_go_right(_last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Walking,
        Action::Falling(n) => Action::Falling(n),
        Action::Walking => Action::Walking,
        Action::Jabbing => Action::Walking,
        Action::NAiring(n) => Action::Falling(n),
    };
    (Orientation::Right, a)
}

fn modify_go_left(_last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Walking,
        Action::Falling(n) => Action::Falling(n),
        Action::Walking => Action::Walking,
        Action::Jabbing => Action::Walking,
        Action::NAiring(n) => Action::Falling(n),
    };
    (Orientation::Left, a)
}

fn modify_jab(last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Jabbing,
        Action::Falling(n) => Action::NAiring(n),
        Action::Walking => Action::Jabbing,
        Action::Jabbing => Action::Jabbing,
        Action::NAiring(n) => Action::NAiring(n),
    };
    (last_o, a)
}

fn new_stance(
    last_o: Orientation,
    last_a: Action,
    intent: &Intent,
) -> Option<(Orientation, Action)> {
    let res = match intent.0 {
        IntentKind::Neutral => modify_neutral(last_o, last_a),
        IntentKind::GoRight => modify_go_right(last_o, last_a),
        IntentKind::GoLeft => modify_go_left(last_o, last_a),
        IntentKind::Jab => modify_jab(last_o, last_a),
        IntentKind::LeftTilt => modify_jab(last_o, last_a), // TODO fix
        IntentKind::RightTilt => modify_jab(last_o, last_a), // TODO fix
        _ => {
            unimplemented!()
        }
    };
    // TODO: push this down even deeper to save computation
    if res.0 != last_o || res.1 != last_a {
        Some(res)
    } else {
        None
    }
}

pub fn set_stance_system(mut query: Query<(&mut FightingStance, &Intent)>) {
    log::debug!("Setting stances");
    for (mut stance, intent) in query.iter_mut() {
        let mut unchanged = true;
        if stance.countdown >= 0 {
            stance.countdown -= 1;
            log::trace!("Counting up stance");
        } else if let Some((new_o, new_a)) = new_stance(stance.orientation, stance.action, intent) {
            unchanged = false;
            stance.orientation = new_o;
            stance.action = new_a;
            log::debug!("Switching to new stance {:?}", *stance);
        }
        if unchanged {
            log::trace!("Counting up stance");
            stance.countup = stance.countup.wrapping_add(1);
        } else {
            log::trace!("Initializing stance countup and countdown");
            stance.countdown = num_countdown_frames(stance.action);
            stance.countup = 0;
        }
        log::debug!("Stance is {:?}", stance);
    }
}
