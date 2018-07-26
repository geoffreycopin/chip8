use sdl2::keyboard::Scancode;

use std::{
    collections::{HashMap}
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct KeyPad {
    pressed: [bool; 16],
    maping: HashMap<Scancode, u8>,
}

impl KeyPad {
    pub fn new() -> KeyPad {
        let maping = vec![
            (Scancode::Num1, 1),
            (Scancode::Num2, 2),
            (Scancode::Num3, 3),
            (Scancode::Num4, 0xC),
            (Scancode::A, 4),
            (Scancode::Z, 5),
            (Scancode::E, 6),
            (Scancode::R, 0xD),
            (Scancode::Q, 7),
            (Scancode::S, 8),
            (Scancode::D, 9),
            (Scancode::F, 0xE),
            (Scancode::W, 0xA),
            (Scancode::X, 0),
            (Scancode::C, 0xB),
            (Scancode::V, 0xF),
        ].into_iter()
            .collect();
        KeyPad::with_maping(maping).unwrap()
    }

    pub fn with_maping(maping: HashMap<Scancode, u8>) -> Option<KeyPad> {
        if maping.values().any(|code| *code > 0xF) {
            return None;
        }
        Some(KeyPad { maping, pressed: [false; 16] })
    }

    pub fn is_pressed(&self, num: u8) -> bool {
        if num > 0xF {
            false
        } else  {
            self.pressed[num as usize]
        }
    }

    pub fn key_down(&mut self, scancode: Scancode) {
        if let Some(&code) = self.maping.get(&scancode) {
            self.pressed[code as usize] = true;
        }
    }

    pub fn key_up(&mut self, scancode: Scancode) {
        if let Some(&code) = self.maping.get(&scancode) {
            self.pressed[code as usize] = false;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_maping_code() {
        assert_eq!(None,
                   KeyPad::with_maping(vec![(Scancode::Num0, 0xFF)].into_iter().collect()));
    }

    #[test]
    fn key_down() {
        let mut pad = KeyPad::new();
        assert_eq!(false, pad.is_pressed(1));
        pad.key_down(Scancode::Num1);
        assert!(pad.is_pressed(1));
    }

    #[test]
    fn key_up() {
        let mut pad = KeyPad::new();
        pad.key_down(Scancode::Num1);
        pad.key_up(Scancode::Num1);
        assert_eq!(false, pad.is_pressed(1));
    }
}
