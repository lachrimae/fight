use crate::machine::environment::types as environment;

pub struct EvilPostbox {}

pub static evil_postbox: EvilPostbox = EvilPostbox {};

impl environment::Characteristics for EvilPostbox {
    fn countdown(&self, state: &environment::State) -> i8 {
        -1
    }
}
