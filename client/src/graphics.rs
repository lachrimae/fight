use crate::world::*;
use bevy::prelude::*;

pub fn update_graphics_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.);
    }
}
