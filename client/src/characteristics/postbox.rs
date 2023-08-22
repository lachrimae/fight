use crate::machine::environment::types as environment;
use crate::machine::grounded::types as grounded;

pub struct Postbox {}

pub static POSTBOX: Postbox = Postbox {};

impl environment::Characteristics for Postbox {
    fn countdown(&self, _state: &environment::State) -> i8 {
        -1
    }
}

impl grounded::Characteristics for Postbox {
    fn countdown(&self, _state: &grounded::State) -> i8 {
        -1
    }
}
