use std::sync::mpsc::{self, Sender};
use std::thread;

use enigo::{Enigo, Key, KeyboardControllable};
use tpp_core::button::{Button, ButtonEvent, ButtonState};

const SELECT: Key = Key::Layout('x');
const START: Key = Key::Layout('c');

const UP: Key = Key::UpArrow;
const DOWN: Key = Key::DownArrow;
const LEFT: Key = Key::LeftArrow;
const RIGHT: Key = Key::RightArrow;

const A: Key = Key::Layout('a');
const B: Key = Key::Layout('b');

const L: Key = Key::Layout('l');
const R: Key = Key::Layout('r');

pub fn spawn_handler() -> Sender<ButtonEvent> {
    let (tx, rx) = mpsc::channel::<ButtonEvent>();

    thread::spawn(move || {
        let rx = rx;

        let mut enigo = Enigo::new();

        for event in rx {
            let ButtonEvent { button, state } = event;

            let key = map_button(button);
            match state {
                ButtonState::Up => enigo.key_up(key),
                ButtonState::Down => enigo.key_down(key),
            }
        }
    });

    tx
}

fn map_button(button: Button) -> Key {
    match button {
        Button::Select => SELECT,
        Button::Start => START,
        Button::Up => UP,
        Button::Down => DOWN,
        Button::Left => LEFT,
        Button::Right => RIGHT,
        Button::A => A,
        Button::B => B,
        Button::L => L,
        Button::R => R,
    }
}
