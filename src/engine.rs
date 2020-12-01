use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use specs::prelude::*;

use crate::ecs::animation::*;
use crate::ecs::collision::*;
use crate::ecs::components::*;
use crate::ecs::enemy::*;
use crate::ecs::player::*;
use crate::ecs::renderer;
use crate::ecs::resources::*;
use crate::ecs::systems::*;
use crate::ecs::weapon::*;

use crate::input;

pub struct Engine {
    window_width: u32,
    window_height: u32,
}

impl Engine {
    pub fn new(width: u32, height: u32) -> Self {
        Engine {
            window_width: width,
            window_height: height,
        }
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
                .load_texture(crate::assets::PLAYER_SPRITE_PATH)
                .expect("could not load texture"),
            texture_creator
                .load_texture(crate::assets::BOSS_SPRITE_PATH)
                .expect("could not load texture"),
            texture_creator
                .load_texture(crate::assets::BULLET_SPRITE_PATH)
                .expect("could not load texture"),
            texture_creator
                .load_texture(crate::assets::EXPLOSION_SPRITE_PATH)
                .expect("could not load texture"),
        ];

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut input = input::Input::new();

        // Register systems
        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem, "input", &[])
            .with(PositionUpdateSystem, "position updater", &[])
            .with(CollisionSystem::new(), "collision", &[])
            .with(WeaponSystem, "weapon system", &[])
            .with(LifetimeKiller, "lifetime", &[])
            .with(HealthSystem, "health", &[])
            .with(AnimationSystem, "animation", &[])
            .with(EnemySystem, "enemy", &[])
            .with(EnemySpawnerSystem::default(), "enemy spawner", &[])
            .with(PlayerRespawnSystem, "player spawner", &[])
            .build();

        // Register required components
        let mut world = World::new();
        world.register::<Weapon>();
        world.register::<Lifetime>();
        world.register::<Enemy>();
        world.register::<Player>();
        world.register::<Projectile>();
        world.register::<Damage>();
        dispatcher.setup(&mut world);
        renderer::SystemData::setup(&mut world);

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

            canvas.present();
            let loop_duration = std::time::Instant::now() - start_time;

            if input.get_key(sdl2::keyboard::Scancode::F12).held {
                println!("FPS {}", 1.0 / loop_duration.as_secs_f64());
            }
        } // Loop
    }
}
