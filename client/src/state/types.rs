pub use crate::state::events::UserInput;

#[derive(Copy, Clone)]
pub enum ArmourLevel {
    NoArmour,
    HyperArmour,
    Invincible,
}

pub struct FrameState {
    pub armour_level: ArmourLevel,
}

pub struct FrameContext {}

// Here we apply the visitor pattern.
// We expect the kinds of events which can be registered to be somewhat constrained.
// User inputs, collisions and timers are the main sorts of events we can expect to occur.
// But we can imagine that new characters will need to have an extensible collection of

#[bevy_reflect::reflect_trait]
pub trait State {
    fn visit_user_input(&self, context: &FrameContext, input: UserInput) -> Option<Box<dyn State>>;
    fn visit_timeout(&self, context: &FrameContext) -> Option<Box<dyn State>>;
    fn num_frames(&self, context: &FrameContext) -> i8;
    fn frame_schedule(&self, frame: u8) -> FrameState;
}

pub trait Event {
    fn accept(&self, context: &FrameContext, visitor: &dyn State) -> Option<Box<dyn State>>;
}
