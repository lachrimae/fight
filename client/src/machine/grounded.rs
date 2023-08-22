use super::common::{
    DefenseState, Machine, MachineContext, MachineInput, MachineResult, PhysicsEvent,
};
use crate::characteristics::{evil_postbox, postbox, Character};


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
        _input: &mut MachineInput,
    ) -> MachineResult {
        MachineResult::Remain
    }

    fn consume_physics_event(
        &mut self,
        _context: &MachineContext,
        _physics: &PhysicsEvent,
    ) -> MachineResult {
        MachineResult::Remain
    }

    fn defense_state(&self) -> DefenseState {
        DefenseState::Normal
    }
}
