use crate::characteristics::{postbox, Character};
use bevy::log;
use std::collections::HashMap;

pub enum AerialAction {
    Hopping,
    Jumping,
    Falling,
    FastFalling,
    CharacterAction,
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum DefenseState {
    #[default]
    Normal,
    HyperArmour,
    Invincible,
}

pub enum PhysicsEvent {
    FellOffPlat,
    LandedOnPlat,
    GotHit,
}

type Key = ();
type InputDiff = ();

pub struct MachineInput(HashMap<Key, InputDiff>);

impl MachineInput {
    pub fn consume(&mut self, key: &Key) -> Option<InputDiff> {
        self.0.remove(&key)
    }
}

pub struct TransitionResult {
    pub children: Option<Vec<Box<dyn Machine>>>,
    pub countdown: i8,
}

pub enum MachineResult {
    Remain,
    Transition(TransitionResult),
}

pub trait Machine {
    fn consume_input(
        &mut self,
        context: &MachineContext,
        input: &mut MachineInput,
    ) -> MachineResult;
    fn consume_physics_event(
        &mut self,
        context: &MachineContext,
        event: &PhysicsEvent,
    ) -> MachineResult;
    fn defense_state(&self) -> DefenseState;
}

pub struct MachineContext {
    pub character: Character,
    pub countdown: i8,
    pub countup: u8,
}

struct HierarchicalMachine {
    machine_stack: Vec<Box<dyn Machine>>,
    context: MachineContext,
}

impl HierarchicalMachine {
    pub fn consume_input(&mut self, input: &mut MachineInput) {
        for machine in self.machine_stack.iter_mut().rev() {
            match machine.consume_input(&self.context, input) {
                MachineResult::Remain => {
                    self.context.countup += 1;
                    if self.context.countdown >= 0 {
                        self.context.countdown -= 1;
                    }
                }
                MachineResult::Transition(transition) => {
                    transition.children.map(|v| {
                        unimplemented!();
                    });
                    self.context.countdown = transition.countdown;
                    self.context.countup = 0;
                }
            }
        }
    }

    pub fn defense_state(&self) -> DefenseState {
        self.machine_stack
            .iter()
            .map(|m| m.defense_state())
            .max()
            .unwrap_or_default()
    }
}
