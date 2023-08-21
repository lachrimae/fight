use crate::world::{
    Accelerating, Acceleration, Action, Fighter, FightingStance, Intent, IntentKind, Moving,
    Orientation, Platform, Position, Velocity,
};
use bevy::log;
use bevy::prelude::*;

use crate::action;

// TODO: make this depend on character
const TERMINAL_VELOCITY: i32 = 20;

pub fn set_physical_props_system(
    mut query: Query<(&mut Velocity, &mut Acceleration, &FightingStance, &Intent)>,
) {
    log::debug!("physical props system beginning");
    for (mut vel, mut acc, stance, intent) in query.iter_mut() {
        if action::stops_movement(stance.action) {
            acc.x = 0;
            acc.y = 0;
            vel.x = 0;
            vel.y = 0;
        } else if action::is_aerial(stance.action) {
            acc.y = -1;
            match intent.0 {
                IntentKind::GoRight => {
                    vel.x = 2;
                }
                IntentKind::GoLeft => {
                    vel.x = -2;
                }
                IntentKind::CrawlRight => {
                    vel.x = 2;
                    acc.y -= 2;
                }
                IntentKind::CrawlLeft => {
                    vel.x = -2;
                    acc.y -= 2;
                }
                IntentKind::Crouch => {
                    acc.y -= 2;
                }
                _ => {}
            }
        }
        if stance.action == Action::Walking {
            acc.x = 0;
            match stance.orientation {
                Orientation::Left => {
                    vel.x = -3;
                }
                Orientation::Right => {
                    vel.x = 3;
                }
            }
        } else if let Action::Jumping(_) = stance.action {
            log::trace!("Jumping!");
            vel.y = 18;
        }
        // TODO: account for other directions of movement
        if vel.y.abs() > TERMINAL_VELOCITY {
            if vel.y < 0 {
                vel.y = -TERMINAL_VELOCITY;
            } else {
                vel.y = TERMINAL_VELOCITY;
            }
            acc.y = 0;
        }
        log::trace!("velocity is now {:?}", vel);
        log::trace!("acceleration is now {:?}", acc);
    }
}

const FIGHTER_DIMENSIONS: i32 = 40;

fn fighter_is_on_plat(pos: &Position, plat: &Platform) -> bool {
    if pos.x < plat.x + plat.width
        && pos.x + FIGHTER_DIMENSIONS > plat.x
        && pos.y < plat.y + 1
        && pos.y + FIGHTER_DIMENSIONS > plat.y
    {
        log::trace!("Character at {:?} standing on platform at {:?}", pos, plat);
        true
    } else {
        log::trace!(
            "Character at {:?} not standing on platform at {:?}",
            pos,
            plat
        );
        false
    }
}

fn first_collision(
    position: &Position,
    velocity: &Velocity,
    platform_query: &Query<&Platform>,
) -> Option<Position> {
    let mut result: Option<Position> = None;
    for platform in platform_query.iter() {
        let x_diff = velocity.x.signum();
        let y_diff = velocity.y.signum();
        if x_diff == 0 && y_diff == 0 {
            continue;
        }
        let mut test_x = 0;
        let mut test_y = 0;
        if x_diff == 0 || y_diff == 0 {
            while !fighter_is_on_plat(
                &Position {
                    x: position.x + test_x,
                    y: position.y + test_y,
                },
                platform,
            ) {
                test_x += x_diff;
                test_y += y_diff;
                if test_x.abs() > velocity.x.abs() || test_y.abs() > velocity.y.abs() {
                    break;
                }
            }
        } else {
            log::trace!(
                "Beginning collision loop with vars x_diff:{:?}, y_diff:{:?} and max velocity x:{:?}, y:{:?}",
                x_diff,
                y_diff,
                velocity.x,
                velocity.y,
            );
            let mut count = 0;
            // TODO: make this loop a little bit smarter.
            while !fighter_is_on_plat(
                &Position {
                    x: position.x + test_x,
                    y: position.y + test_y,
                },
                platform,
            ) {
                count += 1;
                if count > 200 {
                    panic!();
                }
                if (test_y * velocity.x).abs() >= (velocity.y * test_x).abs() {
                    test_x += x_diff;
                } else {
                    test_y += y_diff;
                }
                if test_x.abs() > velocity.x.abs() || test_y.abs() > velocity.y.abs() {
                    break;
                }
            }
        }
        if let Some(ref pos) = result {
            if pos.x - position.x > test_x && pos.y - position.y > test_y {
                result = Some(Position {
                    x: position.x + test_x,
                    y: position.y + test_y,
                });
            }
        } else {
            result = Some(Position {
                x: position.x + test_x,
                y: position.y + test_y,
            });
        }
    }
    result
}

pub fn movement_system(
    mut fighter_query: Query<
        (&mut Position, &Velocity, &FightingStance),
        (With<Moving>, With<Fighter>),
    >,
    platform_query: Query<&Platform>,
) {
    log::debug!("movement system beginning");
    for (mut position, velocity, stance) in &mut fighter_query {
        if matches!(stance.action, Action::Jumping(_)) {
            log::trace!("Player jumping");
            position.x += velocity.x;
            position.y += velocity.y;
        } else {
            let col = first_collision(&position, &velocity, &platform_query);
            if let Some(col_position) = col {
                log::trace!("Player movement obstructed");
                *position = col_position;
            } else {
                log::trace!("Player moving unobstructed");
                position.x += velocity.x;
                position.y += velocity.y;
            }
        }
        log::trace!("position is now {:?}", position);
    }
}

pub fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration), With<Accelerating>>) {
    log::debug!("acceleration system beginning");
    for (mut velocity, acceleration) in &mut query {
        velocity.x += acceleration.x;
        velocity.y += acceleration.y;
        log::trace!("velocity is now {:?}", velocity);
    }
}
