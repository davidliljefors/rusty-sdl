use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use specs::prelude::*;

use crate::ecs::collision::*;
use crate::ecs::components::*;
use crate::ecs::renderer;
use crate::ecs::resources::*;
use crate::ecs::systems::*;
use crate::input;

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn run(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context
            .video()
            .expect("Could not initiate video sybsystem");
        let _image_context =
            image::init(InitFlag::PNG | InitFlag::JPG).expect("could not make image context");

        let window = video_subsystem
            .window("rust-sdl2 demo", 1600, 900)
            .position_centered()
            .build()
            .expect("could not make window");

        let mut canvas = window.into_canvas().build().expect("could not make canvas");
        let texture_creator = canvas.texture_creator();

        let textures = [
            texture_creator
                .load_texture("assets/Bullethellplayer.png")
                .expect("could not load texture"),
            texture_creator
                .load_texture("assets/BullethellBoss.png")
                .expect("could not load texture"),
            texture_creator
                .load_texture("assets/bullet.png")
                .expect("could not load texture"),
        ];
        let player_sprite_id: usize = 0;
        let enemy_sprite_id: usize = 1;
        let _bullet_sprite_id: usize = 2;

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        
        let mut input = input::Input::new();

        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem, "input", &[])
            .with(PositionUpdateSystem, "position updater", &["input"])
            .with(CollisionSystem, "collision", &["position updater"])
            .with(PositionPrinterSystem, "pos printer", &[])
            .with(WeaponSystem, "weapon system", &["input"])
            .with(LifetimeKiller, "lifetime", &[])
            .build();

        let mut world = World::new();
        world.register::<Weapon>();
        world.register::<Lifetime>();
        dispatcher.setup(&mut world);
        renderer::SystemData::setup(&mut world);

        world
            .create_entity()
            .with(Position { x: 800.0, y: 300.0 })
            .with(Sprite {
                spritesheet: enemy_sprite_id,
                size: sdl2::rect::Point::new(128, 128),
                src_rect: sdl2::rect::Rect::new(0, 0, 128, 128),
            })
            .with(CircleCollider {
                radius: 32.0,
                id: 1,
            })
            .with(Name {
                name: String::from("Enemy"),
            })
            .build();

        // Create player
        world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(Sprite {
                spritesheet: player_sprite_id,
                size: sdl2::rect::Point::new(128, 128),
                src_rect: sdl2::rect::Rect::new(0, 0, 128, 128),
            })
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(CircleCollider {
                radius: 32.0,
                id: 2,
            })
            .with(Name {
                name: String::from("Player"),
            })
            .with(Weapon {
                speed: 1400.0,
                time_between_shots: 0.2,
                cooldown: 0.0,
                wants_to_fire: false,
            })
            .with(KeyboardControlled {})
            .build();

        let mut last_frame_time = std::time::Instant::now();

        let mut event_pump = sdl_context.event_pump().unwrap();
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
                println!("DeltaTime {}", elapsed_time.as_secs_f32() );
            }

            {
                // Update resource state for input
                let mut input_resource = world.write_resource::<InputResource>();

                // future warning expensive input copy
                *input_resource = InputResource(input);
                //                         here ^^^^^
            }

            dispatcher.dispatch(&world);
            world.maintain();

            renderer::render(
                &mut canvas,
                Color::RGB(0, 0, 0),
                &textures,
                world.system_data(),
            )
            .expect("Render failed");

            canvas.present();
        }
    }
}
