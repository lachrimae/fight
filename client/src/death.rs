use crate::world::{Fighter, Position, Stocks};
use bevy::log;
use bevy::prelude::*;

pub fn death_system(
    mut commands: Commands,
    mut query: Query<(&mut Position, &mut Stocks, Entity), With<Fighter>>,
) {
    for (mut position, mut stocks, entity) in query.iter_mut() {
        if position.x.abs() > 720 || position.y.abs() > 720 {
            log::debug!("Character dying");
            stocks.count -= 1;
            if stocks.count <= 0 {
                commands.entity(entity).despawn();
                log::debug!("Out of stocks, despawned");
            } else {
                position.x = 0;
                position.y = 90;
                log::debug!("Down to {:?} stocks, respawning", stocks.count);
            }
        }
    }
}
