use crate::machine::environment::types as environment;
use crate::machine::grounded::types as grounded;

pub struct EvilPostbox {}

pub static EVIL_POSTBOX: EvilPostbox = EvilPostbox {};

impl environment::Characteristics for EvilPostbox {
    fn countdown(&self, state: &environment::State) -> i8 {
        -1
    }
}

impl grounded::Characteristics for EvilPostbox {
    fn countdown(&self, state: &grounded::State) -> i8 {
        -1
    }
}
