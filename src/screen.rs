#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Pixel {
    x: usize,
    y: usize,
    on: bool,
}

impl Pixel {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn on(&self) -> bool {
        self.on
    }
}

pub struct Screen {
    pixels: [Pixel; 2048],
}

impl Screen {
    pub fn new() -> Screen {
        let mut pixels = [Pixel::default(); 2048];
        for y in 0..32 {
            for x in 0..64 {
                let p = &mut pixels[(y * 64) + x];
                p.x = x;
                p.y = y;
            }
        }
        Screen { pixels }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|p| p.on = false)
    }

    pub fn pixels(&self) -> impl Iterator<Item = &Pixel> {
        self.pixels.iter()
    }

    pub fn set_pixel_value(&mut self, x: usize, y: usize, on: bool) -> bool {
        let index = ((y % 32) * 64) + (x % 64);
        if on && self.pixels[index].on  {
            self.pixels[index].on = false;
            return true
        } else {
            self.pixels[index].on = on;
            return false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn new() {
        let s = Screen::new();
        let pixels: Vec<&Pixel> = s.pixels().collect();
        let mut idx = 0;
        for y in 0..32 {
            for x in 0..64 {
                assert_eq!(x, pixels[idx].x());
                assert_eq!(y, pixels[idx].y());
                idx += 1;
            }
        }
    }

    #[test]
    fn turn_on() {
        for y in 0..32 {
            if random() {
                for x in 0..64 {
                    let mut screen = Screen::new();
                    let collision = screen.set_pixel_value(x, y, true);
                    let on: Vec<&Pixel> = screen.pixels.iter().filter(|x| x.on()).collect();
                    assert_eq!(false, collision);
                    assert_eq!(1, on.len());
                    assert_eq!(&Pixel { x, y, on: true }, on[0]);
                }
            }
        }
    }

    #[test]
    fn turn_off() {
        for y in 0..32 {
            if random() {
                for x in 0..64 {
                    let mut screen = all_on_screen();
                    let collision = screen.set_pixel_value(x, y, false);
                    let off: Vec<&Pixel> = screen.pixels.iter().filter(|x| !x.on()).collect();
                    assert_eq!(false, collision);
                    assert_eq!(1, off.len());
                    assert_eq!(&Pixel { x, y, on: false }, off[0]);
                }
            }
        }
    }

    #[test]
    fn turn_on_collision() {
        let mut s = Screen::new();
        s.set_pixel_value(5, 5, true);
        assert!(s.set_pixel_value(5, 5, true));
    }

    #[test]
    fn clear() {
        let mut s = Screen::new();
        s.set_pixel_value(17, 21, true);
        s.set_pixel_value(63, 31, true);
        s.set_pixel_value(0, 0, true);
        s.clear();
        for p in s.pixels.iter() {
            assert!(!p.on)
        }
    }

    fn all_on_screen() -> Screen {
        let mut s = Screen::new();
        for y in 0..32 {
            for x in 0..64 {
                s.set_pixel_value(x, y, true);
            }
        }
        s
    }
}
