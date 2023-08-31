use crate::state::standing::Standing;
use crate::state::types::*;
use bevy_reflect::prelude::*;

#[derive(Reflect)]
#[reflect(State)]
pub struct Punching;
impl State for Punching {
    fn visit_user_input(
        &self,
        _context: &FrameContext,
        _input: UserInput,
    ) -> Option<Box<dyn State>> {
        Some(Box::new(Standing {}))
    }
    fn visit_timeout(&self, _context: &FrameContext) -> Option<Box<dyn State>> {
        Some(Box::new(Standing))
    }
    fn num_frames(&self, _context: &FrameContext) -> i8 {
        13
    }
    fn frame_schedule(&self, _frame: u8) -> FrameState {
        FrameState {
            armour_level: ArmourLevel::NoArmour,
        }
    }
}
