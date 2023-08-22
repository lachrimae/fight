use crate::machine::environment::types::{EnvironmentCharacteristics, EnvironmentState};

pub struct Postbox {}

impl EnvironmentCharacteristics for Postbox {
    fn countdown(&self, state: &EnvironmentState) -> i8 {
        -1
    }
}
