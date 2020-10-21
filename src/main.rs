use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::{Scancode};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{event::Event, render::Canvas, render::Texture, video::Window};

use std::time::Duration;

const TILE_SIZE: u32 = 64;

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ButtonState {
    pressed: bool,
    released: bool,
    held: bool,
}
#[derive(Clone, Copy)]
struct MouseState {
    held:bool
}

#[allow(dead_code)]
struct Input {
    key_state: [ButtonState; Scancode::Num as usize],
    mouse_state: [MouseState; 8],
    mouse_pos: (i32, i32),
}

#[allow(dead_code)]
impl Input {
    fn get_key(&self, code: Scancode) -> ButtonState {
        self.key_state[code as usize]
    }
    fn get_mouse_pos() {}

    fn get_mouse_button(&self, btn: MouseButton) -> MouseState {
        self.mouse_state[btn as usize]
    }

    fn new() -> Input {
        let button = ButtonState {
            pressed: false,
            released: false,
            held: false,
        };
        let mouse = MouseState {held:false};
        Input {
            key_state: [button; Scancode::Num as usize],
            mouse_state: [mouse; 8],
            mouse_pos: (0, 0),
        }
    }
}

fn render(canvas: &mut Canvas<Window>, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas
        .copy(
            texture,
            player.sprite,
            Rect::new(
                player.position.x(),
                player.position.y(),
                TILE_SIZE,
                TILE_SIZE,
            ),
        )
        .expect("faild to draw texture");
    Ok(())
}

pub fn main() {
    println!("Number of keys {}", Scancode::Num as i32);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context =
        image::init(InitFlag::PNG | InitFlag::JPG).expect("could not make image context");

    let window = video_subsystem
        .window("rust-sdl2 demo", 1600, 900)
        .position_centered()
        .build()
        .expect("could not make window");

    let mut canvas = window.into_canvas().build().expect("could not make canvas");
    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .load_texture("assets/character_sheet.png")
        .expect("could not load texture");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 16, 18),
    };

    let mut input = Input::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                Event::KeyDown {
                    scancode,
                    ..
                } => {
                    if let Some(scancode) = scancode {
                        input.key_state[scancode as usize].held = true;
                    }
                }

                Event::KeyUp {
                    scancode,
                    ..
                } => {
                    if let Some(scancode) = scancode {
                        input.key_state[scancode as usize].held = false;
                    }
                }

                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => { input.mouse_pos = (x,y); }

                Event::MouseButtonDown {
                    mouse_btn, ..
                } => {
                    input.mouse_state[mouse_btn as usize].held = true;
                }

                Event::MouseButtonUp {
                    mouse_btn, ..
                } => {
                    input.mouse_state[mouse_btn as usize].held = false;
                }
                _ => {}
            }
        }

        if input.get_mouse_button(MouseButton::Left).held {
            println!("Mouse is held at pos x={} y={}", input.mouse_pos.0, input.mouse_pos.1);
        }

        render(&mut canvas, &texture, &player).expect("could not render");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
