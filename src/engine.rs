use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use specs::prelude::*;
use rand::Rng;

use crate::ecs::animation::*;
use crate::ecs::collision::*;
use crate::ecs::components::*;
use crate::ecs::renderer;
use crate::ecs::resources::*;
use crate::ecs::systems::*;
use crate::input;

pub struct Engine {
    window_width: u32,
    window_height: u32,
    target_fps: u32,
}

impl Engine {
    pub fn new(width: u32, height: u32) -> Self {
        Engine {
            window_width: width,
            window_height: height,
            target_fps: 60,
        }
    }

    pub fn desired_frame_duration(&self) -> std::time::Duration {
        std::time::Duration::new(0, 1_000_000_000 / self.target_fps)
    }

    pub fn run(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context
            .video()
            .expect("Could not initiate video sybsystem");
        let _image_context =
            image::init(InitFlag::PNG | InitFlag::JPG).expect("could not make image context");

        let window = video_subsystem
            .window("rusty-sdl", self.window_width, self.window_height)
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
            texture_creator
                .load_texture("assets/explo.png")
                .expect("could not load texture"),
        ];
        let player_sprite_id: usize = 0;
        let enemy_sprite_id: usize = 1;
        let _bullet_sprite_id: usize = 2;
        let explosion_sprite_id: usize = 3;

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
            .with(
                ResponseSystem::default(),
                "collision response",
                &["collision"],
            )
            .with(HealthSystem, "health", &["collision response"])
            .with(AnimationSystem, "animation", &[])
            .build();

        let mut world = World::new();
        world.register::<Weapon>();
        world.register::<Lifetime>();
        dispatcher.setup(&mut world);
        renderer::SystemData::setup(&mut world);

        // Create enemy
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
                layer: LayerMask::from_enum(Layers::Enemy),
            })
            .with(CollisionResponse::new())
            .with(Name {
                name: String::from("Enemy"),
            })
            .with(Health::new(100))
            .with(Damage { damage: 10 })
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
                layer: LayerMask::from_enum(Layers::Player),
            })
            .with(CollisionResponse::new())
            .with(Name {
                name: String::from("Player"),
            })
            .with(Weapon {
                speed: 1400.0,
                time_between_shots: 0.2,
                cooldown: 0.0,
                wants_to_fire: false,
                damage: 5,
            })
            .with(Health::new(100))
            .with(KeyboardControlled {})
            .build();

        let mut last_frame_time = std::time::Instant::now();

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            let start_time = std::time::Instant::now();

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
                let elapsed_time = elapsed_time.as_secs_f32();
                let mut delta_time = world.write_resource::<DeltaTime>();
                *delta_time = DeltaTime(elapsed_time);

                last_frame_time = std::time::Instant::now();
            }

            {
                // Update resource state for input
                let mut input_resource = world.write_resource::<InputResource>();
                *input_resource = InputResource(input);
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
            
            let loop_duration = std::time::Instant::now() - start_time;
            let desired_duration = self.desired_frame_duration();

            if loop_duration > desired_duration {
                println!("Stuttering!!");
            } else {
                let wait_time = desired_duration - loop_duration;
                std::thread::sleep(wait_time);
            }
            if input.get_key(sdl2::keyboard::Scancode::F12).held {
                println!("Momentary FPS {}", 1.0 / loop_duration.as_secs_f32())
            }

            canvas.present();
        } // Loop
    }
}
