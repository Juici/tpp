use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Button {
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    L,
    R,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ButtonState {
    Up,
    Down,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Up
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ButtonEvent {
    pub state: ButtonState,
    pub button: Button,
}
