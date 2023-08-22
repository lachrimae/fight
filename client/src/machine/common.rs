use crate::characteristics::{postbox, Character};
use bevy::log;
use std::collections::HashMap;

// TODO: at some point, profile the use of Box<dyn Machine> here. If it is too expensive,
// come up with a way of using &dyn Machine and allocating via an object pool.

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
    pub context: MachineContext,
}

enum HierarchicalInput<'a> {
    MachineInput(&'a mut MachineInput),
    PhysicsEvent(&'a mut PhysicsEvent),
}

impl HierarchicalMachine {
    fn consume_thing(mut self, input: &mut HierarchicalInput) -> Self {
        let mut index = self.machine_stack.len() - 1;
        while index >= 0 {
            let res = {
                let machine = &mut self.machine_stack[index];
                match input {
                    HierarchicalInput::MachineInput(i) => machine.consume_input(&self.context, *i),
                    HierarchicalInput::PhysicsEvent(e) => {
                        machine.consume_physics_event(&self.context, *e)
                    }
                }
            };
            match res {
                MachineResult::Remain => {
                    self.context.countup += 1;
                    if self.context.countdown >= 0 {
                        self.context.countdown -= 1;
                    }
                }
                MachineResult::Transition(mut transition) => {
                    transition.children.as_mut().map(|mut v| {
                        self.machine_stack.truncate(index + 1);
                        self.machine_stack.append(&mut v);
                    });
                    self.context.countdown = transition.countdown;
                    self.context.countup = 0;
                }
            }
            index -= 1;
        }
        self
    }
    pub fn consume_input(mut self, input: &mut MachineInput) -> Self {
        self.consume_thing(&mut HierarchicalInput::MachineInput(input))
    }

    pub fn consume_physics_event(mut self, event: &mut PhysicsEvent) -> Self {
        self.consume_thing(&mut HierarchicalInput::PhysicsEvent(event))
    }

    pub fn defense_state(&self) -> DefenseState {
        self.machine_stack
            .iter()
            .map(|m| m.defense_state())
            .max()
            .unwrap_or_default()
    }
}
