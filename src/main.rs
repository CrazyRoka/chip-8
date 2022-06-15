use std::time::Duration;

use anyhow::{anyhow, Result};
use chip_8::{
    self,
    input::keyboard::{Key, Keyboard},
    models::chip8::{Chip8, CHIP8_HEIGHT, CHIP8_WIDTH},
};
use sdl2::{event::Event, keyboard::Keycode, pixels, rect::Rect, EventPump};

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let rom = std::fs::read(filename)?;
    let mut chip = Chip8::new(rom);

    let sdl_context = sdl2::init().map_err(|err| anyhow!(err))?;
    let video_subsystem = sdl_context.video().map_err(|err| anyhow!(err))?;
    let window = video_subsystem
        .window("Chip8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(pixels::Color::BLACK);
    canvas.clear();
    canvas.present();

    let sleep_duration = Duration::from_millis(2);
    let mut events = sdl_context.event_pump().map_err(|err| anyhow!(err))?;

    loop {
        let keyboard = poll(&mut events)?;

        let result = chip.emulateCycle(&keyboard)?;
        if result.draw_update {
            for (y, row) in result.gfx.iter().enumerate() {
                for (x, &pixel) in row.iter().enumerate() {
                    let x = (x as u32) * SCALE_FACTOR;
                    let y = (y as u32) * SCALE_FACTOR;

                    canvas.set_draw_color(color(pixel));
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))
                        .map_err(|err| anyhow!(err))?;
                }
            }
            canvas.present();
        }

        std::thread::sleep(sleep_duration);
    }
}

fn poll(events: &mut EventPump) -> Result<Keyboard> {
    if events.poll_iter().any(|event| {
        if let Event::Quit { .. } = event {
            true
        } else {
            false
        }
    }) {
        return Err(anyhow!("User closed application. Shutting down"));
    }

    let keys: Vec<Keycode> = events
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    let mut keyboard = Keyboard::new();

    for key in keys {
        if let Ok(keyboard_key) = Key::parse(key) {
            keyboard.press(keyboard_key);
        }
    }

    Ok(keyboard)
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::BLUE
    } else {
        pixels::Color::GREEN
    }
}
