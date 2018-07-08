extern crate rand;
extern crate sdl2;

mod cpu;
mod keypad;
mod opcodes;
mod screen;

use screen::Screen;

use sdl2::{event::Event, rect, render::Canvas, video::Window};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip-8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mut s = screen::Screen::new();
        s.turn_on(0, 0);
        s.turn_on(0, 1);
        s.turn_on(1, 1);
        s.turn_on(63, 31);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        draw_screen(&mut canvas, &s);
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { .. } => break 'running,
                _ => (),
            }
        }
    }
}

fn draw_screen(canvas: &mut Canvas<Window>, screen: &Screen) {
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
