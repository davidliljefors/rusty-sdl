use sdl2::keyboard::Scancode;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::*;

pub struct PositionPrinterSystem;

impl<'a> System<'a> for PositionPrinterSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Name>);

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::SystemData) {
        //let (pos, name) = data;

        // for (pos, name) in (&pos, &name).join() {
        //     println!("{:?} is at {:?}", &name.name, &pos);
        // }
    }
}

pub struct LifetimeKiller;

impl<'a> System<'a> for LifetimeKiller {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Lifetime>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut lifetime, elapsed_time) = data;
        let elapsed_time = elapsed_time.0;
        for (entity, lifetime) in (&entities, &mut lifetime).join() {
            lifetime.time_left -= elapsed_time;
            if lifetime.time_left < 0.0 {
                entities
                    .delete(entity)
                    .expect("error deleting after lifetime ended")
            }
        }
    }
}

pub struct WeaponSystem;

impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Weapon>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, delta, position, mut weapon, updater): Self::SystemData) {
        let delta = delta.0;

        let handle_weapon = |(_entity, position, weapon): (Entity, &Position, &mut Weapon)| {
            weapon.cooldown -= delta;

            if weapon.wants_to_fire && weapon.cooldown <= 0.0 {
                let projectile = entities.create();
                weapon.cooldown = weapon.time_between_shots;

                updater.insert(
                    projectile,
                    Position {
                        x: position.x,
                        y: position.y,
                    },
                );
                updater.insert(
                    projectile,
                    Sprite {
                        spritesheet: 2,
                        size: sdl2::rect::Point::new(32, 32),
                        src_rect: sdl2::rect::Rect::new(0, 0, 16, 16),
                    },
                );
                updater.insert(
                    projectile,
                    Velocity {
                        x: 0.0,
                        y: -weapon.speed as f32,
                    },
                );
                updater.insert(projectile, Lifetime { time_left: 0.5 });
                updater.insert(projectile, CircleCollider { radius: 16.0, id:3 });
                updater.insert(projectile, Name{name:String::from("Projectile")});
            }
        };

        (&entities, &position, &mut weapon)
            .join()
            .for_each(handle_weapon);
    }
}

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, InputResource>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Weapon>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, controlled, mut velocity, mut weapon) = data;
        let input = &input.0;

        for (velocity, _) in (&mut velocity, &controlled).join() {
            velocity.y = 0.0;
            velocity.x = 0.0;
        }

        if input.get_key(Scancode::Up).held {
            for (velocity, _) in (&mut velocity, &controlled).join() {
                velocity.y = -400.0;
            }
        }

        if input.get_key(Scancode::Down).held {
            for (velocity, _) in (&mut velocity, &controlled).join() {
                velocity.y = 400.0;
            }
        }

        if input.get_key(Scancode::Left).held {
            for (velocity, _) in (&mut velocity, &controlled).join() {
                velocity.x = -400.0;
            }
        }

        if input.get_key(Scancode::Right).held {
            for (velocity, _) in (&mut velocity, &controlled).join() {
                velocity.x = 400.0;
            }
        }

        let should_fire = input.get_key(Scancode::Space).held;
        for (weapon, _) in (&mut weapon, &controlled).join() {
            weapon.wants_to_fire = should_fire;
        }
    }
}
