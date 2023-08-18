use crate::action;
use crate::world::*;
use bevy::log;
use bevy::prelude::*;
use std::option::Option;

fn num_countdown_frames(action: Action) -> i8 {
    match action {
        Action::Standing => -1,
        Action::Jumping(_) => 0,
        Action::Falling(_) => -1,
        Action::Walking => -1,
        Action::Jabbing => 13,
        Action::NAiring(_) => 40,
    }
}

fn modify_neutral(last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Standing,
        Action::Jumping(n) => Action::Falling(n),
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
        Action::Jumping(n) => Action::Falling(n),
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
        Action::Jumping(n) => Action::Falling(n),
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
        Action::Jumping(n) => Action::NAiring(n),
        Action::Falling(n) => Action::NAiring(n),
        Action::Walking => Action::Jabbing,
        Action::Jabbing => Action::Jabbing,
        Action::NAiring(n) => Action::NAiring(n),
    };
    (last_o, a)
}

fn modify_jump(last_o: Orientation, last_a: Action) -> (Orientation, Action) {
    let a = match last_a {
        Action::Standing => Action::Jumping(Jumps(0)),
        Action::Jumping(Jumps(0)) => Action::Jumping(Jumps(1)),
        Action::Jumping(Jumps(1)) => Action::Jumping(Jumps(2)),
        Action::Jumping(Jumps(2)) => Action::Falling(Jumps(2)),
        Action::Falling(Jumps(0)) => Action::Jumping(Jumps(1)),
        Action::Falling(Jumps(1)) => Action::Jumping(Jumps(2)),
        Action::Falling(Jumps(2)) => Action::Falling(Jumps(2)),
        Action::Walking => Action::Jumping(Jumps(0)),
        Action::Jabbing => Action::Jumping(Jumps(0)),
        Action::NAiring(Jumps(0)) => Action::Jumping(Jumps(1)),
        Action::NAiring(Jumps(1)) => Action::Jumping(Jumps(2)),
        Action::NAiring(Jumps(2)) => Action::Falling(Jumps(2)),
        _ => {
            log::error!("Jump count too high!");
            Action::Falling(Jumps(2))
        }
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
        IntentKind::Jump => modify_jump(last_o, last_a),
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

const FIGHTER_DIMENSIONS: i32 = 40;

fn fighter_is_in_air(pos: &Position, query: &Query<&Platform>) -> bool {
    for plat in query.iter() {
        if pos.x < plat.x + plat.width
            && pos.x + FIGHTER_DIMENSIONS > plat.x
            && pos.y < plat.y + 1
            && pos.y + FIGHTER_DIMENSIONS > plat.y
        {
            log::trace!("Character at {:?} standing on platform at {:?}", pos, plat);
            return false;
        } else {
            log::trace!(
                "Character at {:?} not standing on platform at {:?}",
                pos,
                plat
            );
        }
    }
    true
}

pub fn set_stance_system(
    mut fighter_query: Query<(&mut FightingStance, &Position, &Intent)>,
    platform_query: Query<&Platform>,
) {
    log::debug!("Setting stances");
    for (mut stance, position, intent) in fighter_query.iter_mut() {
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
        let should_fall = fighter_is_in_air(position, &platform_query);
        let is_aerial = action::is_aerial(stance.action);
        let is_jump = matches!(stance.action, Action::Jumping(_));
        if should_fall && !is_aerial {
            log::trace!("Character fell!");
            stance.action = Action::Falling(Jumps(0));
            stance.countup = 0;
            stance.countdown = -1;
        } else if !should_fall && is_aerial && !is_jump {
            log::trace!("Character landed!");
            stance.action = Action::Standing;
            stance.countup = 0;
            stance.countdown = -1;
        }
        log::debug!("Stance is {:?}", stance);
    }
}
