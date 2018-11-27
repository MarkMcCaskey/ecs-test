use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{self, Sdl};

use nalgebra::Point2;

use components::*;
use specs::prelude::*;

pub struct RenderSystem {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl RenderSystem {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("specs test 0", 720, 540)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .unwrap();

        Self {
            canvas,
            sdl_context,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw_box(&mut self, loc: Point2<f32>) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas
            .draw_point(Point::new(loc.x as i32, loc.y as i32))
            .unwrap();
    }

    pub fn draw(&mut self) {
        self.canvas.present();
    }

    pub fn handle_events(&mut self, world: &mut World) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => ::std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    //game.toggle_state();
                }
                Event::MouseButtonDown { x, y, .. } => {
                    world
                        .create_entity()
                        .with(Vel::new(0.1, 0.1))
                        .with(Pos::new(x as f32, y as f32))
                        .build();
                }
                Event::TextEditing {
                    text,
                    start,
                    length,
                    ..
                } => println!("{:?}, {:?}, {:?}", text, start, length),

                Event::TextInput { text, .. } => println!("text input {}", text),

                _ => {}
            }
        }
    }
}
