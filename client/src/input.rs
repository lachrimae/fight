use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[repr(u8)]
pub enum DiscreteInput {
    Jump,
    Hit,
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum InputState {
    NotActivated = 0,
    Activated = 1,
}

fn to_input_state(n: u8) -> InputState {
    match n {
        0 => InputState::NotActivated,
        1 => InputState::Activated,
        _ => panic!("Invalid InputState representation"),
    }
}

const BITS_PER_INPUT: u8 = 1;

const fn get_shift(input: DiscreteInput) -> u8 {
    BITS_PER_INPUT * input as u8
}

const BASE_MASK: u8 = 1;

const fn shift_mask(input: DiscreteInput) -> u8 {
    BASE_MASK << get_shift(input)
}

fn shift_flag(input: DiscreteInput, diff: InputState) -> u8 {
    let flag = diff as u8;
    let shift = get_shift(input);
    log::trace!("input::shift_flag: shifting {:?} left by {:?}", diff, shift);
    flag << shift
}

// TODO: Make this more complicated when there are multiple local players
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct CombinedInput(u8);

impl CombinedInput {
    pub fn new() -> Self {
        CombinedInput(0)
    }

    pub fn get(&self, button: DiscreteInput) -> InputState {
        let shift = get_shift(button);
        let mask = shift_mask(button);
        let flag = (self.0 & mask) >> shift;
        to_input_state(flag)
    }

    pub fn set(&mut self, button: DiscreteInput, state: ButtonState) {
        let next_state = match state {
            ButtonState::Pressed => InputState::Activated,
            ButtonState::Released => InputState::NotActivated,
        };
        let mask = shift_mask(button);
        let flag = shift_flag(button, next_state);
        self.0 = (self.0 & !mask) | (flag & mask);
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
        KeyCode::A => Some(DiscreteInput::Left),
        KeyCode::S => Some(DiscreteInput::Down),
        KeyCode::D => Some(DiscreteInput::Right),
        KeyCode::W => Some(DiscreteInput::Jump),
        KeyCode::Space => Some(DiscreteInput::Hit),
        _ => None,
    }
}

pub fn input_system(
    In(_handle): In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> CombinedInput {
    log::debug!("Registering inputs");
    let mut input = CombinedInput::new();
    // We want older events to have precedence over newer ones, thus reverse iterate
    for keycode in KEYCODES_OF_INTEREST {
        if keyboard_input.pressed(*keycode) {
            input.set(keycode_mapper(keycode).unwrap(), ButtonState::Pressed);
        }
    }
    log::debug!("{:#?}", input);
    input
}
