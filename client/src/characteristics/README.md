We can take the following as an example characteristics file. This one indicates to a given state machine for how long the "environment" actions should last for. In this case, the function `countdown` signals a `-1` in all instances, representing that "environment" actions are indefinite. This pattern will repeat for more complex examples, for example the length of time that stunlocks or aerial attacks should last for, as well as any additional information these moves need to carry.
```rust
use crate::machine::environment::types as environment;

pub struct Postbox {}

pub static postbox: Postbox = Postbox {};

impl environment::Characteristics for Postbox {
    fn countdown(&self, _state: &environment::State) -> i8 {
        -1
    }
}
```
Some of the choices here may appear strange. The empty struct, and its static initialization, exist for the sole purpose of constructing a `&'static dyn environment::Characteristics` trait object at runtime. This trait object is then used by the state machine `client/src/machine/environment.rs` to determine how long various actions should take.
