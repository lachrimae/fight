use super::common::{
    DefenseState, Machine, MachineContext, MachineInput, MachineResult, PhysicsEvent,
    TransitionResult,
};
use crate::characteristics::{postbox, Character};
use bevy::log;

pub mod types;
use types::{EnvironmentCharacteristics, EnvironmentState, Jumps};

impl Machine for EnvironmentState {
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
        let characteristics: Box<dyn EnvironmentCharacteristics> = match context.character {
            Character::Postbox => Box::new(postbox::Postbox {}),
        };

        match self {
            EnvironmentState::Aerial(_) => {
                match physics {
                    PhysicsEvent::FellOffPlat => {
                        log::warn!("Fell off platform while aerial");
                        MachineResult::Remain
                    }
                    PhysicsEvent::LandedOnPlat => {
                        *self = EnvironmentState::Grounded;
                        MachineResult::Transition(TransitionResult {
                            // TODO: give it a landing animation child
                            children: vec![].into(),
                            countdown: characteristics.countdown(self),
                        })
                    }
                    PhysicsEvent::GotHit => {
                        unimplemented!();
                    }
                }
            }
            EnvironmentState::Grounded => match physics {
                PhysicsEvent::FellOffPlat => {
                    *self = EnvironmentState::Aerial(Jumps(1));
                    MachineResult::Transition(TransitionResult {
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
