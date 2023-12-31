use crate::world::Action;

pub fn is_aerial(action: Action) -> bool {
    match action {
        Action::Standing => false,
        Action::Jumping(_) => true,
        Action::Falling(_) => true,
        Action::Walking => false,
        Action::Jabbing => false,
        Action::NAiring(_) => true,
    }
}

pub fn stops_movement(action: Action) -> bool {
    match action {
        Action::Standing => true,
        Action::Jumping(_) => false,
        Action::Falling(_) => false,
        Action::Walking => false,
        Action::Jabbing => true,
        Action::NAiring(_) => false,
    }
}
