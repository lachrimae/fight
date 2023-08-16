
use crate::input::{CombinedInput, DiscreteInput};
use crate::types::*;
use crate::world::{Allegiance, Intent, IntentKind};
use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::{PlayerInputs};

fn mk_command(input: CombinedInput) -> Intent {
    let inner = {
        let is_left =
            input.is_pressed(DiscreteInput::Left) && !input.is_pressed(DiscreteInput::Right);
        let is_right =
            input.is_pressed(DiscreteInput::Right) && !input.is_pressed(DiscreteInput::Left);
        if input.is_pressed(DiscreteInput::Down) {
            if input.is_pressed(DiscreteInput::Hit) {
                IntentKind::DownTilt
            } else if is_right {
                IntentKind::CrawlRight
            } else if is_left {
                IntentKind::CrawlLeft
            } else {
                IntentKind::Crouch
            }
        } else if is_left {
            if input.is_pressed(DiscreteInput::Hit) {
                IntentKind::LeftTilt
            } else {
                IntentKind::GoLeft
            }
        } else if is_right {
            if input.is_pressed(DiscreteInput::Hit) {
                IntentKind::RightTilt
            } else {
                IntentKind::GoRight
            }
        } else if input.is_pressed(DiscreteInput::Hit) {
            IntentKind::Jab
        } else if input.is_pressed(DiscreteInput::Jump) {
            IntentKind::Jump
        } else {
            IntentKind::Neutral
        }
    };
    Intent(inner)
}

pub fn set_intent_system(
    keyboard_input: Res<PlayerInputs<GgrsConfig>>,
    mut query: Query<(&mut Intent, &Allegiance)>,
) {
    log::debug!("Setting intent");
    for (mut intent, allegiance) in query.iter_mut() {
        *intent = mk_command(keyboard_input[allegiance.handle].0);
        log::debug!("Player {:?} has intent {:?}", allegiance, intent);
    }
}
