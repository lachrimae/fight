use crate::input::{Button, InputState};
use crate::types::*;
use crate::world::{is_being_pressed, Allegiance, ButtonDiff, InputDiff, Intent, IntentKind};

use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;
use strum::IntoEnumIterator;

fn to_button_diff(n: u16) -> ButtonDiff {
    let res = match n {
        0 => ButtonDiff::NotHeld,
        1 => ButtonDiff::Held,
        2 => ButtonDiff::Released,
        3 => ButtonDiff::Pressed,
        _ => panic!(),
    };
    log::trace!("input::to_button_diff: Converting {:?} to {:?}", n, res);
    res
}

const BITS_PER_INPUT: u16 = 2;

const fn get_shift(input: Button) -> u16 {
    BITS_PER_INPUT * input as u16
}

const BASE_MASK: u16 = 3;

fn shift_mask(input: Button) -> u16 {
    BASE_MASK << get_shift(input)
}

fn shift_flag(input: Button, diff: ButtonDiff) -> u16 {
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
impl InputDiff {
    pub fn new() -> Self {
        InputDiff(0)
    }

    pub fn get(self, button: Button) -> ButtonDiff {
        let shift = get_shift(button);
        let mask = shift_mask(button);
        let masked_val = self.0 & mask;
        let flag = masked_val >> shift;
        log::trace!(
            "InputDiff::get: masking {:?} to get {:?} then shifting right by {:?} to get {:?}",
            self.0,
            masked_val,
            shift,
            flag
        );
        to_button_diff(flag)
    }

    pub fn set(&mut self, button: Button, state: InputState) {
        let next_diff = if self.is_being_pressed(button) {
            match state {
                InputState::Activated => ButtonDiff::Held,
                InputState::NotActivated => ButtonDiff::Released,
            }
        } else {
            match state {
                InputState::Activated => ButtonDiff::Pressed,
                InputState::NotActivated => ButtonDiff::NotHeld,
            }
        };
        let mask = shift_mask(button);
        let flag = shift_flag(button, next_diff);
        self.0 = (self.0 & !mask) | (flag & mask);
    }

    pub fn is_being_pressed(self, button: Button) -> bool {
        let diff = self.get(button);
        is_being_pressed(diff)
    }
}

fn mk_diff(old_diff: ButtonDiff, new_input: InputState) -> ButtonDiff {
    match (is_being_pressed(old_diff), new_input) {
        (true, InputState::Activated) => ButtonDiff::Held,
        (true, InputState::NotActivated) => ButtonDiff::Released,
        (false, InputState::Activated) => ButtonDiff::Pressed,
        (false, InputState::NotActivated) => ButtonDiff::NotHeld,
    }
}

pub fn input_diff_system(
    keyboard_input: Res<PlayerInputs<GgrsConfig>>,
    mut query: Query<(&mut InputDiff, &Allegiance)>,
) {
    log::debug!("Registering input diffs");
    for (mut input_diff, allegiance) in query.iter_mut() {
        log::debug!("input diff: {:#?}", input_diff);
        let new_input = keyboard_input[allegiance.handle.0];
        for button in Button::iter() {
            input_diff.set(button, new_input.0.get(button));
        }
    }
}
