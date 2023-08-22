pub struct Jumps(pub u8);

pub enum State {
    Aerial(Jumps),
    Grounded,
}

pub trait Characteristics {
    fn countdown(&self, state: &State) -> i8;
}
