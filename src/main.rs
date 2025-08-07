extern crate sdl2;

use tetris_sdl2::frame_manager::{self, FrameManager};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{FRect, Rect};
use std::time::Duration;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("tetris-sdl", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    const TETRIMINOS: [[[u8; 4]; 2]; 7] = [
        //0
        [   [1, 1, 0, 0], 
            [1, 1, 0, 0]
        ],
        //i
        [
            [0, 0, 0, 0], 
            [1, 1, 1, 1]
        ],
        //t
        [
            [0, 1, 0, 0], 
            [1, 1, 1, 0]
        ],
        //l
        [
            [0, 0, 1, 0], 
            [1, 1, 1, 0]
        ],
        //j
        [
            [1, 0, 0, 0], 
            [1, 1, 1, 0]
        ],
        //s
        [
            [0, 1, 1, 0], 
            [1, 1, 0, 0]
        ],
        //z
        [
            [1, 1, 0, 0], 
            [0, 1, 1, 0]
        ]
    ];
    let mut rect = FRect::new(250.0, 10.0, 50.0, 50.0);
    let rect_color = Color::RGB(150, 150, 55);
    const H_SPEED: f32 = 50.0;
    const V_SPEED: f32 = 50.0;

    let mut accumulator = 0.0;
    let mut v_delta = 0.0;
    let dt = 0.016;

    let mut fm = FrameManager::new(60);
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    let key = keycode.unwrap();
                    if key == Keycode::Left {
                        rect.reposition((rect.x - (H_SPEED), rect.y));
                    }
                    if key == Keycode::Right {
                        rect.reposition((rect.x + (H_SPEED), rect.y));
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        let delta = fm.delta_time().as_secs_f32();
        let delta = delta.min(0.05);
        v_delta += dt;
        accumulator += delta;
        while accumulator >= dt {
            accumulator -= dt;
        }
        if v_delta >= 0.8 {
            if rect.height() + rect.y < 600.0 {
                rect.reposition((rect.x, (rect.y + V_SPEED).min(600.0)));
            }
            v_delta = 0.0;
        }
        canvas.set_draw_color(rect_color);
        let _ = canvas.fill_frect(rect);
        canvas.present();
        fm.delay_to_maintain_fps();
    }
}
