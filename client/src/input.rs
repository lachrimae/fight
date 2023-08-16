use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;
use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DiscreteInput {
    Jump,
    Hit,
    Left,
    Right,
    Down,
}

const fn shift_flag(input: DiscreteInput) -> u8 {
    1 << (input as u8)
}

// TODO: Make this more complicated when there are multiple local players
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct CombinedInput(u8);

#[derive(Debug, Resource)]
pub struct LocalInputs(pub HashMap<PlayerHandle, CombinedInput>);

impl CombinedInput {
    pub fn new() -> Self {
        CombinedInput(0)
    }

    pub fn set(&mut self, button: DiscreteInput, state: ButtonState) {
        let flag = shift_flag(button);
        match state {
            ButtonState::Pressed => self.0 |= flag,
            ButtonState::Released => self.0 &= !flag,
        };
    }

    pub const fn is_pressed(&self, button: DiscreteInput) -> bool {
        self.0 << shift_flag(button) != 0
    }
}

const KEYCODES_OF_INTEREST: &[KeyCode] = &[
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::W,
    KeyCode::Space,
];

const fn keycode_mapper(keycode: &KeyCode) -> Option<DiscreteInput> {
    match keycode {
        (KeyCode::A) => Some(DiscreteInput::Left),
        (KeyCode::S) => Some(DiscreteInput::Down),
        (KeyCode::D) => Some(DiscreteInput::Right),
        (KeyCode::W) => Some(DiscreteInput::Jump),
        (KeyCode::Space) => Some(DiscreteInput::Hit),
        _ => None,
    }
}

pub fn input(In(_handle): In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> CombinedInput {
    log::debug!("Registering inputs");
    let mut input = CombinedInput::new();
    // We want older events to have precedence over newer ones, thus reverse iterate
    for keycode in KEYCODES_OF_INTEREST {
        if keyboard_input.pressed(*keycode) {
            input.set(keycode_mapper(keycode).unwrap(), ButtonState::Pressed);
        }
    }
    log::info!("{:#?}", input);
    input
}
