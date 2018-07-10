use sdl2::keyboard::Scancode;

use std::{
    collections::{HashMap, HashSet}, iter::FromIterator,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct KeyPad {
    pressed: HashSet<u8>,
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

    pub fn with_maping(mut maping: HashMap<Scancode, u8>) -> Option<KeyPad> {
        if maping.values().any(|code| *code > 0xF) {
            return None;
        }
        Some(KeyPad { maping, pressed: HashSet::new() })
    }

    pub fn update<T: Iterator<Item = Scancode>>(&mut self, pressed: T) {
        self.pressed = pressed
            .filter_map(|code| self.maping.get(&code))
            .cloned()
            .collect();
    }

    pub fn is_pressed(&self, num: u8) -> bool {
        self.pressed.contains(&num)
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
    fn update() {
        let mut pad = KeyPad::new();
        pad.update(vec![Scancode::Num1, Scancode::Z].into_iter());
        for i in 0..16 {
            if i == 1 || i == 5 {
                assert!(pad.is_pressed(i));
            } else {
                assert!(! pad.is_pressed(i))
            }
        }
    }
}
