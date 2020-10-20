use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::rect::Rect;
use std::time::Duration;

const BOARD_WIDTH: u32 = 3;
const BOARD_SIZE: usize = (BOARD_WIDTH * BOARD_WIDTH) as usize;
const TILE_SIZE: u32 = 128;
const BOARD_OFFSET: u32 = 0;
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Empty,
    Cross,
    Circle,
}

#[derive(Clone, Copy)]
pub struct Tile {
    rect: Rect,
    tile_type: TileType,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn window_pos_to_index(x: i32, y: i32) -> Option<usize> {
    let mut idx: i32 = x - BOARD_OFFSET as i32;
    let mut idy: i32 = y - BOARD_OFFSET as i32;
    idx /= TILE_SIZE as i32;
    idy /= TILE_SIZE as i32;

    let result = idx + idy * BOARD_WIDTH as i32;

    println!("We returend {}", result);

    if result < BOARD_SIZE as i32 && result >= 0 {
        Some(result as usize)
    } else {
        None
    }
}

fn check_for_win(board: &[Tile]) -> TileType {
    // Check lines
    for i in (0..9).step_by(3) {
        if board[i].tile_type == board[i + 1].tile_type
            && board[i + 1].tile_type == board[i + 2].tile_type
        {
            return board[i].tile_type;
        }
    }

    // Check rows
    for i in 0..3 {
        if board[i].tile_type == board[i + 3].tile_type
            && board[i + 3].tile_type == board[i + 6].tile_type
        {
            return board[i].tile_type;
        }
    }

    // Check diagonals
    if board[0].tile_type == board[4].tile_type && board[4].tile_type == board[8].tile_type {
        return board[0].tile_type;
    }

    if board[2].tile_type == board[4].tile_type && board[4].tile_type == board[6].tile_type {
        return board[2].tile_type;
    }

    TileType::Empty
}

fn is_board_full(board: &[Tile]) -> bool {
    for tile in board {
        if tile.tile_type == TileType::Empty {
            return false;
        }
    }
    true
}

fn init_board(board: &mut [Tile]) {
    let mut x: u32 = BOARD_OFFSET;
    let mut y: u32 = BOARD_OFFSET;
    for tile in board.iter_mut() {
        tile.rect.set_x(x as i32);
        tile.rect.set_y(y as i32);
        x += TILE_SIZE;
        if x / TILE_SIZE >= 3 {
            x = BOARD_OFFSET;
            y += TILE_SIZE;
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const WINDOW_SIZE: u32 = 2 * BOARD_OFFSET + 3 * TILE_SIZE;

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let temp_rect = sdl2::rect::Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
    let default_rect = Tile {
        rect: temp_rect,
        tile_type: TileType::Empty,
    };

    let mut board_pieces: [Tile; BOARD_SIZE] = [default_rect; BOARD_SIZE];
    init_board(&mut board_pieces);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut current_player: TileType = TileType::Cross;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for tile in &board_pieces {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            if tile.tile_type == TileType::Empty {
                canvas.draw_rect(tile.rect).unwrap();
            } else if tile.tile_type == TileType::Cross {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(tile.rect).unwrap();
            } else if tile.tile_type == TileType::Circle {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(tile.rect).unwrap();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        println!("x = {}, y = {}", x, y);
                        let index_or = window_pos_to_index(x, y);

                        if let Some(index) = index_or {
                            if board_pieces[index].tile_type == TileType::Empty {
                                board_pieces[index].tile_type = current_player;

                                let winner = check_for_win(&board_pieces);

                                if winner == TileType::Cross {
                                    println!("Green win!");
                                    std::process::exit(0);
                                }

                                if winner == TileType::Circle {
                                    println!("Red win!");
                                    std::process::exit(0);
                                }

                                if is_board_full(&board_pieces) {
                                    println!("Its a tie.");
                                    std::process::exit(0);
                                }

                                if current_player == TileType::Circle {
                                    current_player = TileType::Cross
                                } else {
                                    current_player = TileType::Circle;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
