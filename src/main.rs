// #![windows_subsystem = "windows"]
extern crate sdl2;

use rand::seq::SliceRandom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{FRect, Rect};
use tetris_sdl2::frame_manager::{FrameManager, FrameManagerTrait};

pub fn main() {
    // embed_resource::compile("app.rc", embed_resource::NONE);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("tetris-sdl", 640, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut fm = FrameManager::new(144);
    const GRID_UNIT: u16 = 40;
    const LEFT_MARGIN: u16 = GRID_UNIT * 3;
    const START_POS: (usize, usize) = (3, 0);
    const TETRIMINOS: [[[usize; 4]; 2]; 7] = [
        //0
        [[0, 1, 1, 0], [0, 1, 1, 0]],
        //i
        [[0, 0, 0, 0], [1, 1, 1, 1]],
        //t
        [[0, 1, 0, 0], [1, 1, 1, 0]],
        //l
        [[0, 0, 1, 0], [1, 1, 1, 0]],
        //j
        [[1, 0, 0, 0], [1, 1, 1, 0]],
        //s
        [[0, 1, 1, 0], [1, 1, 0, 0]],
        //z
        [[1, 1, 0, 0], [0, 1, 1, 0]],
    ];

    let mut rng = rand::rng();
    let mut tetriniminos_order: [usize; 7] = std::array::from_fn(|i| i as usize);
    tetriniminos_order.shuffle(&mut rng);

    let mut order_index: usize = 0;

    let mut current_tetrimino_pos: (usize, usize) = START_POS;
    let mut game_grid: [[bool; 10]; 20] = std::array::from_fn(|_| std::array::from_fn(|_| false));

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "./assets/Chewy-Regular.ttf"; // đường dẫn tới file .ttf
    let font = ttf_context.load_font(font_path, 28).unwrap(); // size 32 px

    let rect_color = Color::RGB(150, 150, 55);

    let mut accumulator = 0.0;
    let mut v_delta = 0.0;
    let level = 1;
    let mut speed_up = 0;
    let mut stop = false;

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
                Event::KeyDown { keycode, .. } => {
                    let key = keycode.unwrap();
                    if key == Keycode::Left {
                        if current_tetrimino_pos.0 > 0 {
                            current_tetrimino_pos.0 -= 1;
                        }
                    } else if key == Keycode::Right {
                        let current_tetrimino =
                            TETRIMINOS[tetriniminos_order[order_index] as usize];
                        let max_x: usize = 8
                            - current_tetrimino[0][3]
                            - current_tetrimino[1][3]
                            - current_tetrimino[0][2]
                            - current_tetrimino[1][2];
                        if current_tetrimino_pos.0 < max_x {
                            current_tetrimino_pos.0 += 1;
                        }
                    }
                    if key == Keycode::Down {
                        speed_up = 10
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    let key = keycode.unwrap();
                    if key == Keycode::Down {
                        speed_up = 0
                    }
                    if key == Keycode::C {
                        if order_index < 6 {
                            order_index += 1;
                        } else {
                            order_index = 0;
                            tetriniminos_order.shuffle(&mut rng);
                        }
                        current_tetrimino_pos = START_POS;
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn, clicks, ..
                } => if mouse_btn == MouseButton::Left && clicks == 2 {},
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        let delta = fm.delta_time().as_secs_f32();
        let delta = delta.min(0.05);
        v_delta += fm.get_target_fps();
        accumulator += delta;
        while accumulator >= fm.get_target_fps() {
            accumulator -= fm.get_target_fps();
        }
        if v_delta >= (0.8 - ((level + speed_up - 1) as f32) * 0.007).powi(level + speed_up - 1) {
            if current_tetrimino_pos.1 < 18 {
                current_tetrimino_pos.1 += 1;
            } else {
                if order_index < 6 {
                    order_index += 1;
                } else {
                    order_index = 0;
                    tetriniminos_order.shuffle(&mut rng);
                }
                current_tetrimino_pos = START_POS;
            }
            v_delta = 0.0;
        }

        for (ri, row) in game_grid.iter().enumerate() {
            for (ci, value) in row.iter().enumerate() {
                if *value {
                    canvas.set_draw_color(rect_color);
                    let _ = canvas.fill_rect(Rect::new(
                        (ci as i32 * GRID_UNIT as i32) + (LEFT_MARGIN as i32),
                        ri as i32 * GRID_UNIT as i32,
                        GRID_UNIT as u32,
                        GRID_UNIT as u32,
                    ));
                }
                canvas.set_draw_color(Color::BLACK);
                let _ = canvas.draw_rect(Rect::new(
                    (ci as i32 * GRID_UNIT as i32) + (LEFT_MARGIN as i32),
                    ri as i32 * GRID_UNIT as i32,
                    GRID_UNIT as u32,
                    GRID_UNIT as u32,
                ));
            }
        }
        
        let current_tetrimino = TETRIMINOS[tetriniminos_order[order_index] as usize];
        for (ri, row) in current_tetrimino.iter().enumerate() {
            for (ci, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    canvas.set_draw_color(rect_color);
                    let _ = canvas.fill_frect(FRect::new(
                        (LEFT_MARGIN as f32)
                            + (GRID_UNIT as f32 * (current_tetrimino_pos.0 + ci) as f32),
                        GRID_UNIT as f32 * ((ri + current_tetrimino_pos.1) as f32),
                        GRID_UNIT as f32,
                        GRID_UNIT as f32,
                    ));
                    canvas.set_draw_color(Color::BLACK);
                    let _ = canvas.draw_frect(FRect::new(
                        (LEFT_MARGIN as f32)
                            + (GRID_UNIT as f32 * (current_tetrimino_pos.0 + ci) as f32),
                        GRID_UNIT as f32 * ((ri + current_tetrimino_pos.1) as f32),
                        GRID_UNIT as f32,
                        GRID_UNIT as f32,
                    ));
                    if current_tetrimino_pos.1 < 18 {
                        canvas.set_draw_color(Color::RED);
                        let _ = canvas.draw_frect(FRect::new(
                            (LEFT_MARGIN as f32)
                                + (GRID_UNIT as f32 * (current_tetrimino_pos.0 + ci) as f32),
                            GRID_UNIT as f32 * ((ri + 18) as f32),
                            GRID_UNIT as f32,
                            GRID_UNIT as f32,
                        ));
                    }
                    if current_tetrimino_pos.1 == 18 {
                        stop = true;
                        break;
                    } else if current_tetrimino_pos.1 < 18 {
                        let yi = current_tetrimino_pos.1 + 1;
                        let xi = current_tetrimino_pos.0 + ci;
                        if game_grid[yi][xi] {
                            stop = true;
                            break;
                        }
                    }
                }
            }
            if stop {
                break;
            }
        }
        if stop {
            for (ri, row) in current_tetrimino.iter().enumerate() {
                for (ci, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        game_grid[ri + current_tetrimino_pos.1][ci + current_tetrimino_pos.0] = true;
                    }
                }
            }
            stop = false;
        }
        let level_text = format!("level {}", level);
        let texture_creator = canvas.texture_creator();
        let surface = font
            .render(&level_text)
            .solid(Color::RGB(0, 0, 0)) // màu trắng
            .unwrap();

        // Chuyển surface thành texture để vẽ
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let target = sdl2::rect::Rect::new(20, 10, surface.width(), surface.height());
        let _ = canvas.copy(&texture, None, Some(target));

        canvas.present();
        fm.delay_to_maintain_fps();
    }
}
