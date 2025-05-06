use crate::emu::cpu::CPU;
use crate::emu::instruction_mapper;
use crate::emu::memory::Memory;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::sys::SDL_Point;
use sdl2::video::Window;

enum ColorPalette {
    Green,
    LightGreen,
    DarkGreen,
    Black,
}

pub struct Display {
    pixels: Vec<usize>,
}

impl Default for Display {
    fn default() -> Self {
        Self { pixels: initialize_vram() }
    }
}

fn initialize_vram() -> Vec<usize> {
    let len: usize = 32 * 8 * 32 * 8;
    vec![0; len]
}

impl Display {
    fn construct_vram_content(&mut self, memory: &Memory) {
        let tile_start_index: u16 = 0x8000;
        let tile_diff: u16 = 0x0010;
        let row_diff: usize = 32 * 8;
        let mut tile_index: usize = 0;
        for address in 0x9800..=0x9BFF {
            let tile_id = memory.addresses[address as usize];
            let index: u16 = tile_start_index + (tile_diff * tile_id as u16);
            for i in (0..16).step_by(2) {
                let upper = memory.addresses[index as usize + i];
                let lower = memory.addresses[index as usize + i + 1];
                for j in 0..8 {
                    let color = get_bit_at_pos(upper, j) << 1 | get_bit_at_pos(lower, j);
                    let pixel_position = (tile_index / 32) * row_diff * 8 + ((tile_index % 32) * 8) + ((i / 2) * row_diff) + 7 - j as usize;
                    self.pixels[pixel_position] = color as usize;
                }
            }
            tile_index += 1;
        }
    }

    pub fn start_main_process(&mut self, cpu: &mut CPU, memory: &mut Memory) {
        start(self, cpu, memory);
    }
}

fn get_bit_at_pos(byte: u8, pos: u8) -> u8 {
    match pos {
        0 => (byte & 0b00000001) >> 0,
        1 => (byte & 0b00000010) >> 1,
        2 => (byte & 0b00000100) >> 2,
        3 => (byte & 0b00001000) >> 3,
        4 => (byte & 0b00010000) >> 4,
        5 => (byte & 0b00100000) >> 5,
        6 => (byte & 0b01000000) >> 6,
        7 => (byte & 0b10000000) >> 7,
        _ => panic!("Byte index is out of range"),
    }
}

fn map_palette(value: &usize) -> ColorPalette {
    match value {
        0 => ColorPalette::Green,
        1 => ColorPalette::LightGreen,
        2 => ColorPalette::DarkGreen,
        3 => ColorPalette::Black,
        _ => panic!("Invalid color value"),
    }
}

fn get_color(color: ColorPalette) -> Color {
    match color {
        ColorPalette::Green => Color::RGB(155, 188, 15),
        ColorPalette::LightGreen => Color::RGB(139, 172, 15),
        ColorPalette::DarkGreen => Color::RGB(48, 98, 48),
        ColorPalette::Black => Color::RGB(15, 56, 15),
    }
}

fn start(display: &mut Display, cpu: &mut CPU, memory: &mut Memory) {
    let width: u32 = 160;
    let height: u32 = 144;
    let scale = 3;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Redox Damage", width * scale, height * scale).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(get_color(ColorPalette::Green));
    canvas.clear();
    canvas.present();
    let _ = canvas.set_scale(scale as f32, scale as f32);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut count = 0;
    'running: loop {
        if count == 0 {
            render(display, &memory, &mut canvas);
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        instruction_mapper::execute_instruction(cpu, memory);
        update_ly_register(memory, count);
        println!("executing {:2X} at {:2X}", memory.addresses[cpu.pc as usize], cpu.pc);
        count = (count + 1) % 2000;
    }
}

fn render(display: &mut Display, memory: &Memory, canvas: &mut Canvas<Window>) {
    let mut i = 0;
    let line_width = 32 * 8;
    display.construct_vram_content(memory);
    for pixel in display.pixels.iter() {
        canvas.set_draw_color(get_color(map_palette(&pixel)));
        let sdl_point: SDL_Point = SDL_Point { x: i % line_width, y: i / line_width };
        let point: Point = Point::from_ll(sdl_point);
        let _ = canvas.draw_point(point).expect("Render error during drawing pixel");
        i = (i + 1) % display.pixels.len() as i32;
    }
    canvas.present();
}

fn update_ly_register(memory: &mut Memory, index: i32) {
    if index % 10 == 0 {
        memory.addresses[0xFF44] += 1;
    }
    if memory.addresses[0xFF44] >= 154 {
        memory.addresses[0xFF44] = 0;
    }
}
