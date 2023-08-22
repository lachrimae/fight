use super::common::{
    DefenseState, Machine, MachineContext, MachineInput, MachineResult, PhysicsEvent,
    TransitionResult,
};
use crate::characteristics::{evil_postbox, postbox, Character};
use bevy::log;

pub mod types;
use types::{Characteristics, State};

fn get_characteristics(character: Character) -> &'static dyn Characteristics {
    match character {
        Character::Postbox => &postbox::POSTBOX,
        Character::EvilPostbox => &evil_postbox::EVIL_POSTBOX,
    }
}

impl Machine for State {
    fn consume_input(
        &mut self,
        _context: &MachineContext,
        input: &mut MachineInput,
    ) -> MachineResult {
        MachineResult::Remain
    }

    fn consume_physics_event(
        &mut self,
        context: &MachineContext,
        physics: &PhysicsEvent,
    ) -> MachineResult {
        MachineResult::Remain
    }

    fn defense_state(&self) -> DefenseState {
        DefenseState::Normal
    }
}
