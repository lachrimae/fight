use crate::input;
use crate::input::{CombinedInput, DiscreteInput};

// The Command is not the final say
// on the behaviour of the character.
// For example, a character who is falling
// and actives RightTilt will do a FAir or BAir
// depending on their orientation.
pub enum Command {
    GoRight,
    GoLeft,
    Jab,
    RightTilt,
    LeftTilt,
    DownTilt,
    Jump,
    Neutral,
    Crouch,
    CrawlRight,
    CrawlLeft,
}

fn mk_command(input: CombinedInput) -> Command {
    let is_left = input.is_pressed(DiscreteInput::Left) && !input.is_pressed(DiscreteInput::Right);
    let is_right = input.is_pressed(DiscreteInput::Right) && !input.is_pressed(DiscreteInput::Left);
    if input.is_pressed(DiscreteInput::Down) {
        if input.is_pressed(DiscreteInput::Hit) {
            Command::DownTilt
        } else if is_right {
            Command::CrawlRight
        } else if is_left {
            Command::CrawlLeft
        } else {
            Command::Crouch
        }
    } else if is_left {
        if input.is_pressed(DiscreteInput::Hit) {
            Command::LeftTilt
        } else {
            Command::GoLeft
        }
    } else if is_right {
        if input.is_pressed(DiscreteInput::Hit) {
            Command::RightTilt
        } else {
            Command::GoRight
        }
    } else if input.is_pressed(DiscreteInput::Hit) {
        Command::Jab
    } else if input.is_pressed(DiscreteInput::Jump) {
        Command::Jump
    } else {
        Command::Neutral
    }
}
