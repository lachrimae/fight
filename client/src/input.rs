use crate::types::*;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum DiscreteInput {
    Jump = 0,
    Hit = 1,
    Left = 2,
    Right = 3,
    Down = 4,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum InputDiff {
    NotHeld = 0,
    Held = 1,
    Released = 2,
    Pressed = 3,
}

fn to_input_diff(n: u16) -> InputDiff {
    let res = match n {
        0 => InputDiff::NotHeld,
        1 => InputDiff::Held,
        2 => InputDiff::Released,
        3 => InputDiff::Pressed,
        _ => panic!(),
    };
    log::trace!("input::to_input_diff: Converting {:?} to {:?}", n, res);
    res
}

const fn get_shift(input: DiscreteInput) -> u16 {
    2 * input as u16
}

const BASE_MASK: u16 = 3;

fn shift_mask(input: DiscreteInput) -> u16 {
    BASE_MASK << get_shift(input)
}

fn shift_flag(input: DiscreteInput, diff: InputDiff) -> u16 {
    let flag = diff as u16;
    let shift = get_shift(input);
    log::trace!("input::shift_flag: shifting {:?} left by {:?}", diff, shift);
    let res = flag << shift;
    log::trace!(
        "input::shift_flag: {:?} at {:?} is represented as {:?}",
        input,
        diff,
        res
    );
    res
}

// TODO: Make this more complicated when there are multiple local players
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct CombinedInput(u16);

#[derive(Debug, Resource)]
pub struct LocalInputs(pub HashMap<PlayerHandle, CombinedInput>);

impl CombinedInput {
    pub fn new() -> Self {
        CombinedInput(0)
    }

    pub fn get(&self, button: DiscreteInput) -> InputDiff {
        let shift = get_shift(button);
        let mask = shift_mask(button);
        let masked_val = self.0 & mask;
        let flag = masked_val >> shift;
        log::trace!(
            "CombinedInput::get: masking {:?} to get {:?} then shifting right by {:?} to get {:?}",
            self.0,
            masked_val,
            shift,
            flag
        );
        to_input_diff(flag)
    }

    pub fn set(&mut self, button: DiscreteInput, state: ButtonState) {
        let next_diff = if self.is_being_pressed(button) {
            match state {
                ButtonState::Pressed => InputDiff::Held,
                ButtonState::Released => InputDiff::Released,
            }
        } else {
            match state {
                ButtonState::Pressed => InputDiff::Pressed,
                ButtonState::Released => InputDiff::NotHeld,
            }
        };
        let mask = shift_mask(button);
        let flag = shift_flag(button, next_diff);
        self.0 = (self.0 & !mask) | (flag & mask);
    }

    pub fn is_being_pressed(&self, button: DiscreteInput) -> bool {
        let diff = self.get(button);
        match diff {
            InputDiff::NotHeld => false,
            InputDiff::Held => true,
            InputDiff::Released => false,
            InputDiff::Pressed => true,
        }
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
    In(handle): In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
    formatted_input: Res<PlayerInputs<GgrsConfig>>,
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
