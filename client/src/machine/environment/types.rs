pub struct Jumps(pub u8);

pub enum EnvironmentState {
    Aerial(Jumps),
    Grounded,
}

pub trait EnvironmentCharacteristics {
    fn countdown(&self, state: &EnvironmentState) -> i8 {
        -1
    }
}
