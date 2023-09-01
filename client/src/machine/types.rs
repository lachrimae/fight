use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Button {
    Up,
    Left,
    Right,
    Down,
    Punch,
    Jump,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ButtonDiff {
    Pressed,
    Held,
    Released,
    NotHeld,
}

impl ButtonDiff {
    fn is_down(self) -> bool {
        self == ButtonDiff::Pressed || self == ButtonDiff::Held
    }
}

pub struct InputDiff;

impl InputDiff {
    pub fn button_state(&self, _button: Button) -> ButtonDiff {
        ButtonDiff::NotHeld
    }
    pub fn button_is_down(&self, button: Button) -> bool {
        self.button_state(button).is_down()
    }
}

#[derive(Component, Default, Reflect)]
pub enum Armour {
    #[default]
    None,
    HyperArmour,
    Invincibility,
}

#[derive(Component, Default, Reflect)]
pub enum Physics {
    #[default]
    NotMoving,
    Falling,
}
