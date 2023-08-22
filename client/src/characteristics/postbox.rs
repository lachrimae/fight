use crate::machine::environment::types as environment;

pub struct Postbox {}

pub static postbox: Postbox = Postbox {};

impl environment::Characteristics for Postbox {
    fn countdown(&self, state: &environment::State) -> i8 {
        -1
    }
}
