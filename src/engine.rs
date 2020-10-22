use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
#[allow(unused_imports)]
use sdl2::rect::{Point, Rect};
#[allow(unused_imports)]
use sdl2::{event::Event, render::Canvas, render::Texture, video::Window};
#[allow(unused_imports)]
use specs::{
    Builder, DispatcherBuilder, Read, ReadStorage, RunNow, System, World, WorldExt, WriteStorage,
};

use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::ecs::systems::*;
use crate::input;

pub struct TextureManager<'r> {
    creator:sdl2::render::TextureCreator<sdl2::video::WindowContext>,

    textures:Vec<Texture<'r>>
}

impl <'r>TextureManager<'r> {
    fn new(creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {

        Self { creator, textures:vec![] }
    }
    fn add_texture(&mut self, texture:Texture<'r>) -> usize {
        let index = self.textures.len();
        self.textures.push(texture);
        index
    }

    fn make_texture(&mut self, path:&str) {
        let tex = self.creator.load_texture(path).expect("Invalid Texture Filepath");
        
    }

    fn borrow_texture(&self, index:usize) -> &Texture<'r> {
        &self.textures[index]
    }
}



pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn run(&self) {
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

        let texture_creator =  TextureManager::new(canvas.texture_creator());
        //texture_creator.add_texture(texture: Texture<'r>)

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut input = input::Input::new();

        let mut world = World::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with(PositionPrinterSystem, "position printer", &[])
            .with(
                PositionUpdateSystem,
                "position updater",
                &["position printer"],
            )
            .with(InputSystem, "input", &[])
            .build();

        dispatcher.setup(&mut world);

        world
            .create_entity()
            .with(Position { x: 4.0, y: 7.0 })
            .with(Name {
                name: String::from("Ball"),
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Name { name: String::from("Player"),
            })
            .build();

        let mut last_frame_time = std::time::Instant::now();

        'running: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }

                    Event::KeyDown { scancode, .. } => {
                        if let Some(scancode) = scancode {
                            input.key_state[scancode as usize].held = true;
                        }
                    }

                    Event::KeyUp { scancode, .. } => {
                        if let Some(scancode) = scancode {
                            input.key_state[scancode as usize].held = false;
                        }
                    }

                    Event::MouseMotion { x, y, .. } => {
                        input.mouse_pos = (x, y);
                    }

                    Event::MouseButtonDown { mouse_btn, .. } => {
                        input.mouse_state[mouse_btn as usize].held = true;
                    }

                    Event::MouseButtonUp { mouse_btn, .. } => {
                        input.mouse_state[mouse_btn as usize].held = false;
                    }
                    _ => {}
                }
            }

            {
                // Update resource state for elapsed time
                let elapsed_time = std::time::Instant::now() - last_frame_time;
                let mut delta_time = world.write_resource::<DeltaTime>();
                *delta_time = DeltaTime(elapsed_time.as_secs_f32());

                last_frame_time = std::time::Instant::now();
                //println!("DeltaTime {}", elapsed_time.as_secs_f32() );
            }

            {
                // Update resource state for input
                let mut input_resource = world.write_resource::<InputResource>();

                // future warning expensive input copy
                *input_resource = InputResource(input);
                //                          here ^^^^^
            }

            dispatcher.dispatch(&world);
            world.maintain();

            canvas.present();
        }
    }
}
