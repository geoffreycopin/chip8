extern crate rand;
extern crate sdl2;

mod cpu;
mod keypad;
mod opcodes;
mod screen;

use screen::Screen;

use std::{
    io::prelude::*,
    fs::File,
    env,
    time::Duration,
    thread::sleep
};


use sdl2::{event::Event, rect, render::Canvas, video::Window};


fn main() -> std::io::Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let rom_name = env::args().nth(1)
        .expect("usage: ./chip8_emulator <rom_name>");
    let mut rom = File::open(&rom_name)?;
    let mut rom_data = [0u8; 3584];
    rom.read(&mut rom_data)
        .expect("Error while reading the ROM file !");

    let mut c = cpu::Cpu::new();
    c.load_program(&rom_data)
        .expect("Error while loading the ROM !");

    let mut k = keypad::KeyPad::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        c.cycle(&k);
        c.cycle(&k);
        c.cycle(&k);
        c.cycle(&k);
        c.update_timers();

        draw_screen(&mut canvas, &c.screen);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {scancode: s, ..} => if let Some(key) = s { k.key_down(key) },
                Event::KeyUp {scancode: s, ..} => if let Some(key) = s { k .key_up(key) },
                _ => (),
            }
        }

        sleep(Duration::from_millis(10))
    }

    Ok(())
}

fn draw_screen(canvas: &mut Canvas<Window>, screen: &Screen) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

    let (width, height) = canvas.output_size().unwrap();
    let (pixel_width, pixel_height) = (width / 64, height / 32);

    screen.pixels().filter(|p| p.on()).for_each(|p| {
        let x = p.x() * pixel_width as usize;
        let y = p.y() * pixel_height as usize;
        let rectangle = rect::Rect::new(x as i32, y as i32, pixel_width, pixel_height);
        canvas
            .fill_rect(rectangle)
            .expect(&format!("Unable to draw: {:#?}", rectangle));
    })
}
