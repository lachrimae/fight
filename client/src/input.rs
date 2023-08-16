use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscreteInput {
    Jump,
    Hit,
    Left,
    Right,
    Down,
}

// TODO: Make this more complicated when there are multiple local players
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombinedInput {
    pub jump: ButtonState,
    pub hit: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub down: ButtonState,
}

#[derive(Debug, Resource)]
pub struct LocalInputs(pub HashMap<PlayerId, CombinedInput>);

impl CombinedInput {
    pub fn new() -> Self {
        CombinedInput {
            jump: ButtonState::Released,
            hit: ButtonState::Released,
            left: ButtonState::Released,
            right: ButtonState::Released,
            down: ButtonState::Released,
        }
    }

    pub fn set(&mut self, button: DiscreteInput, state: ButtonState) {
        match button {
            DiscreteInput::Jump => {
                self.jump = state;
            }
            DiscreteInput::Hit => {
                self.hit = state;
            }
            DiscreteInput::Left => {
                self.left = state;
            }
            DiscreteInput::Right => {
                self.right = state;
            }
            DiscreteInput::Down => {
                self.down = state;
            }
        }
    }
}

// TODO: Debug unreliable inputs
pub fn register_inputs(
    mut local_input: ResMut<LocalInputs>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    log::debug!("Registering inputs");
    // We want older events to have precedence over newer ones, thus reverse iterate
    for event in keyboard_input_events.iter() {
        let val = match event.key_code {
            Some(KeyCode::A) => Some(DiscreteInput::Left),
            Some(KeyCode::S) => Some(DiscreteInput::Down),
            Some(KeyCode::D) => Some(DiscreteInput::Right),
            Some(KeyCode::W) => Some(DiscreteInput::Jump),
            Some(KeyCode::Space) => Some(DiscreteInput::Hit),
            _ => None,
        };
        if let Some(input) = val {
            local_input
                .0
                .get_mut(&PlayerId(0))
                .unwrap()
                .set(input, event.state);
        }
    }
    log::info!("{:#?}", local_input);
}
