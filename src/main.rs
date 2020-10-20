use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{event::Event, render::Canvas, render::Texture, video::Window};

use std::time::Duration;

struct Player {
    position: Point,
    sprite: Rect,
}

fn render(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    src: Rect,
    dst: Rect,
) -> Result<(), String> {
    canvas
        .copy(texture, src, dst)
        .expect("faild to draw texture");
    canvas.present();
    Ok(())
}

pub fn main() {
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
    let mut clicked_position = Point::new(0,0);
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => if mouse_btn == sdl2::mouse::MouseButton::Left {
                    clicked_position = Point::new(x, y);
                },
                _ => {}
            }
        }

        let src = Rect::new(0, 0, 16, 18);
        let dst = Rect::new(clicked_position.x, clicked_position.y, 64, 64);

        render(&mut canvas, &texture, src, dst).expect("could not render");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
