use crate::machine::postbox::PostboxState;
use crate::world::{Action, FightingStance, ImageAssets, Orientation, Position};
use bevy::log;
use bevy::prelude::*;

pub fn update_graphics_system(
    mut query: Query<(
        &mut Transform,
        &mut Handle<Image>,
        &PostboxState,
        &Position,
        &Orientation,
    )>,
    images: Res<ImageAssets>,
) {
    use crate::machine::postbox::AerialStance as A;
    use crate::machine::postbox::GroundedStance as G;
    use crate::machine::postbox::Stance as S;
    log::debug!("loading sprites again");
    log::debug!("updating sprites");
    for (mut transform, mut sprite, state, position, orientation) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.);
        transform.rotation = match orientation {
            Orientation::Right => Quat::default(),
            Orientation::Left => Quat::from_rotation_y(std::f32::consts::PI),
        };
        *sprite = match state.stance {
            S::Grounded(G::Standing) => images.postbox_stand.clone(),
            S::Grounded(G::Jabbing) => images.postbox_jab.clone(),
            _ => images.postbox_stand.clone(),
        }
    }
}
