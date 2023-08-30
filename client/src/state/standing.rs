use crate::state::punching::Punching;
use crate::state::types::*;
use bevy::log;

pub struct Standing;
impl State for Standing {
    fn visit_user_input(
        &self,
        _context: &FrameContext,
        _input: UserInput,
    ) -> Option<Box<dyn State>> {
        Some(Box::new(Punching {}))
    }
    fn visit_timeout(&self, _context: &FrameContext) -> Option<Box<dyn State>> {
        log::warn!("visit_timeout: Standing can't timeout");
        None
    }
    fn num_frames(&self, _context: &FrameContext) -> i8 {
        -1
    }
    fn frame_schedule(&self, _frame: u8) -> FrameState {
        FrameState {
            armour_level: ArmourLevel::NoArmour,
        }
    }
}
