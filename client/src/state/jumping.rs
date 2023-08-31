use crate::state::standing::Standing;
use crate::state::types::*;
use bevy::log;
use bevy_reflect::prelude::*;

#[derive(Reflect)]
#[reflect(State)]
struct Jumping;
impl State for Jumping {
    fn visit_user_input(
        &self,
        _context: &FrameContext,
        _input: UserInput,
    ) -> Option<Box<dyn State>> {
        None
    }
    fn visit_timeout(&self, _context: &FrameContext) -> Option<Box<dyn State>> {
        log::info!("Jumping not implemented.");
        Some(Box::new(Standing))
    }
    fn num_frames(&self, _context: &FrameContext) -> i8 {
        5
    }
    fn frame_schedule(&self, _frame: u8) -> FrameState {
        FrameState {
            armour_level: ArmourLevel::NoArmour,
        }
    }
}
