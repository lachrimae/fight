use crate::world::{Action, FightingStance, ImageAssets, Orientation, Position};
use bevy::log;
use bevy::prelude::*;

pub fn update_graphics_system(
    mut query: Query<(
        &mut Transform,
        &mut Handle<Image>,
        &FightingStance,
        &Position,
    )>,
    images: Res<ImageAssets>,
) {
    log::debug!("loading sprites again");
    log::debug!("updating sprites");
    for (mut transform, mut sprite, stance, position) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.);
        transform.rotation = match stance.orientation {
            Orientation::Right => Quat::default(),
            Orientation::Left => Quat::from_rotation_y(std::f32::consts::PI),
        };
        *sprite = match stance.action {
            Action::Standing => images.postbox_stand.clone(),
            Action::Walking => images.postbox_walk.clone(),
            Action::Jabbing => images.postbox_jab.clone(),
            _ => images.postbox_stand.clone(),
        }
    }
}
