use std::collections::HashMap;

use sdl2::keyboard::Keycode;

use crate::models::errors::ChipErrors;

#[derive(Debug)]
pub struct Keyboard {
    key_status: HashMap<Key, bool>,
}

impl Keyboard {
    pub fn new() -> Self {
        let mut key_status = HashMap::new();

        key_status.insert(Key::Key1, false);
        key_status.insert(Key::Key2, false);
        key_status.insert(Key::Key3, false);
        key_status.insert(Key::KeyC, false);
        key_status.insert(Key::Key4, false);
        key_status.insert(Key::Key5, false);
        key_status.insert(Key::Key6, false);
        key_status.insert(Key::KeyD, false);
        key_status.insert(Key::Key7, false);
        key_status.insert(Key::Key8, false);
        key_status.insert(Key::Key9, false);
        key_status.insert(Key::KeyE, false);
        key_status.insert(Key::KeyA, false);
        key_status.insert(Key::Key0, false);
        key_status.insert(Key::KeyB, false);
        key_status.insert(Key::KeyF, false);

        Self { key_status }
    }

    pub fn press(&mut self, key: Key) {
        *self.key_status.entry(key).or_default() = true;
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.key_status
            .iter()
            .find(|(_, value)| **value)
            .map(|(key, _)| key.get_code())
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Key1,
    Key2,
    Key3,
    KeyC,
    Key4,
    Key5,
    Key6,
    KeyD,
    Key7,
    Key8,
    Key9,
    KeyE,
    KeyA,
    Key0,
    KeyB,
    KeyF,
}

impl Key {
    pub fn parse(code: sdl2::keyboard::Keycode) -> Result<Key, ChipErrors> {
        let key = match code {
            Keycode::Num1 => Key::Key1,
            Keycode::Num2 => Key::Key2,
            Keycode::Num3 => Key::Key3,
            Keycode::Num4 => Key::KeyC,
            Keycode::Q => Key::Key4,
            Keycode::W => Key::Key5,
            Keycode::E => Key::Key6,
            Keycode::R => Key::KeyD,
            Keycode::A => Key::Key7,
            Keycode::S => Key::Key8,
            Keycode::D => Key::Key9,
            Keycode::F => Key::KeyE,
            Keycode::Z => Key::KeyA,
            Keycode::X => Key::Key0,
            Keycode::C => Key::KeyB,
            Keycode::V => Key::KeyF,
            _ => return Err(ChipErrors::UnknownKeycode(code)),
        };

        Ok(key)
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::KeyC => 0xc,
            Key::Key4 => 0x4,
            Key::Key5 => 0x5,
            Key::Key6 => 0x6,
            Key::KeyD => 0xd,
            Key::Key7 => 0x7,
            Key::Key8 => 0x8,
            Key::Key9 => 0x9,
            Key::KeyE => 0xe,
            Key::KeyA => 0xa,
            Key::Key0 => 0x0,
            Key::KeyB => 0xb,
            Key::KeyF => 0xf,
        }
    }
}
