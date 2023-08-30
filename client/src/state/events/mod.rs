use crate::state::types::*;

#[derive(Copy, Clone)]
pub struct UserInput;
impl Event for UserInput {
    fn accept(&self, context: &FrameContext, visitor: &dyn State) -> Option<Box<dyn State>> {
        visitor.visit_user_input(context, *self)
    }
}
