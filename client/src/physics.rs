use crate::world::*;
use bevy::log;
use bevy::prelude::*;

use crate::action;

pub fn set_physical_props_system(
    mut query: Query<(&mut Velocity, &mut Acceleration, &FightingStance)>,
) {
    log::debug!("physical props system beginning");
    for (mut vel, mut acc, stance) in query.iter_mut() {
        if action::stops_movement(stance.action) {
            acc.x = 0;
            acc.y = 0;
            vel.x = 0;
            vel.y = 0;
        } else if action::is_aerial(stance.action) {
            acc.y = -8;
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
        }
        log::trace!("velocity is now {:?}", vel);
        log::trace!("acceleration is now {:?}", acc);
    }
}

pub fn movement_system(mut query: Query<(&mut Position, &Velocity), With<Moving>>) {
    log::debug!("movement system beginning");
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
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
