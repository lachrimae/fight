use crate::input::{DiscreteInput, InputState};
use crate::types::*;
use crate::world::{
    is_being_pressed, Allegiance, CombinedInputDiff, InputDiff, Intent, IntentKind,
};
use bevy::input::ButtonState;
use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;
use strum::IntoEnumIterator;

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

const BITS_PER_INPUT: u16 = 2;

const fn get_shift(input: DiscreteInput) -> u16 {
    BITS_PER_INPUT * input as u16
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
impl CombinedInputDiff {
    pub fn new() -> Self {
        CombinedInputDiff(0)
    }

    pub fn get(&self, button: DiscreteInput) -> InputDiff {
        let shift = get_shift(button);
        let mask = shift_mask(button);
        let masked_val = self.0 & mask;
        let flag = masked_val >> shift;
        log::trace!(
            "CombinedInputDiff::get: masking {:?} to get {:?} then shifting right by {:?} to get {:?}",
            self.0,
            masked_val,
            shift,
            flag
        );
        to_input_diff(flag)
    }

    pub fn set(&mut self, button: DiscreteInput, state: InputState) {
        let next_diff = if self.is_being_pressed(button) {
            match state {
                InputState::Activated => InputDiff::Held,
                InputState::NotActivated => InputDiff::Released,
            }
        } else {
            match state {
                InputState::Activated => InputDiff::Pressed,
                InputState::NotActivated => InputDiff::NotHeld,
            }
        };
        let mask = shift_mask(button);
        let flag = shift_flag(button, next_diff);
        self.0 = (self.0 & !mask) | (flag & mask);
    }

    pub fn is_being_pressed(&self, button: DiscreteInput) -> bool {
        let diff = self.get(button);
        is_being_pressed(diff)
    }
}

fn mk_diff(old_diff: InputDiff, new_input: InputState) -> InputDiff {
    match (is_being_pressed(old_diff), new_input) {
        (true, InputState::Activated) => InputDiff::Held,
        (true, InputState::NotActivated) => InputDiff::Released,
        (false, InputState::Activated) => InputDiff::Pressed,
        (false, InputState::NotActivated) => InputDiff::NotHeld,
    }
}

pub fn input_diff_system(
    keyboard_input: Res<PlayerInputs<GgrsConfig>>,
    mut query: Query<(&mut CombinedInputDiff, &Allegiance)>,
) {
    log::debug!("Registering input diffs");
    for (mut input_diff, allegiance) in query.iter_mut() {
        log::debug!("input diff: {:#?}", input_diff);
        let new_input = keyboard_input[allegiance.handle.0];
        for button in DiscreteInput::iter() {
            input_diff.set(button, new_input.0.get(button));
        }
    }
}

fn mk_command(input: CombinedInputDiff) -> Intent {
    let inner = {
        let is_left = input.is_being_pressed(DiscreteInput::Left)
            && !input.is_being_pressed(DiscreteInput::Right);
        let is_right = input.is_being_pressed(DiscreteInput::Right)
            && !input.is_being_pressed(DiscreteInput::Left);
        if input.is_being_pressed(DiscreteInput::Down) {
            if input.is_being_pressed(DiscreteInput::Hit) {
                IntentKind::DownTilt
            } else if is_right {
                IntentKind::CrawlRight
            } else if is_left {
                IntentKind::CrawlLeft
            } else {
                IntentKind::Crouch
            }
        } else if is_left {
            if input.is_being_pressed(DiscreteInput::Hit) {
                IntentKind::LeftTilt
            } else {
                IntentKind::GoLeft
            }
        } else if is_right {
            if input.is_being_pressed(DiscreteInput::Hit) {
                IntentKind::RightTilt
            } else {
                IntentKind::GoRight
            }
        } else if input.is_being_pressed(DiscreteInput::Hit) {
            IntentKind::Jab
        } else if matches!(input.get(DiscreteInput::Jump), InputDiff::Pressed) {
            IntentKind::Jump
        } else {
            IntentKind::Neutral
        }
    };
    Intent(inner)
}

pub fn set_intent_system(
    keyboard_input: Res<PlayerInputs<GgrsConfig>>,
    mut query: Query<(&mut Intent, &CombinedInputDiff, &Allegiance)>,
) {
    log::debug!("Setting intents");
    for (mut intent, input_diff, allegiance) in query.iter_mut() {
        *intent = mk_command(*input_diff);
        log::debug!("Player {:?} has intent {:?}", allegiance, intent);
    }
}
