use super::common::{
    DefenseState, Machine, MachineContext, MachineInput, MachineResult, PhysicsEvent,
    TransitionResult,
};
use crate::characteristics::{evil_postbox, postbox, Character};
use crate::machine::grounded::types as grounded;
use bevy::log;
use std::boxed::Box;

pub mod types;
use types::{Characteristics, Jumps, State};

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
        let characteristics = get_characteristics(context.character);

        match self {
            State::Aerial(_) => match physics {
                PhysicsEvent::FellOffPlat => {
                    log::warn!("Fell off platform while aerial");
                    MachineResult::Remain
                }
                PhysicsEvent::LandedOnPlat => {
                    *self = State::Grounded;
                    MachineResult::Transition(TransitionResult {
                        children: vec![Box::new(grounded::State::Landing) as Box<dyn Machine>]
                            .into(),
                        countdown: characteristics.countdown(self),
                    })
                }
                PhysicsEvent::GotHit => {
                    unimplemented!();
                }
            },
            State::Grounded => match physics {
                PhysicsEvent::FellOffPlat => {
                    *self = State::Aerial(Jumps(1));
                    MachineResult::Transition(TransitionResult {
                        // TODO: give a falling animation here
                        children: vec![].into(),
                        countdown: characteristics.countdown(self),
                    })
                }
                _ => unimplemented!(),
            },
        }
    }

    fn defense_state(&self) -> DefenseState {
        DefenseState::Normal
    }
}
