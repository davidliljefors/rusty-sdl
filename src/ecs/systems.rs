use sdl2::keyboard::Scancode;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::ecs::collision::*;


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

pub struct HealthSystem;

impl<'a> System<'a> for HealthSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        
        let entities = data.0;
        let health_storage = data.1;

        for (entity, health) in (&entities, &health_storage).join() {
            if health.health == 0 {
                entities.delete(entity).expect("error deleeting entity")
            }
        }
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
        let (entities, mut lifetime_storage, delta_time) = data;
        let delta_time = delta_time.0;

        for (entity, mut lifetime) in (&entities, &mut lifetime_storage).join() {
            lifetime.time_left -= delta_time;
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

    fn run(&mut self, (entities, delta, position, mut weapon, world): Self::SystemData) {
        let delta = delta.0;

        let handle_weapon = |(_entity, position, weapon): (Entity, &Position, &mut Weapon)| {
            weapon.cooldown -= delta;

            if weapon.wants_to_fire && weapon.cooldown <= 0.0 {
                let projectile = entities.create();
                weapon.cooldown = weapon.time_between_shots;
                
                world.insert(
                    projectile,
                    Projectile{}       
                );
                world.insert(
                    projectile,
                    Position {
                        x: position.x,
                        y: position.y,
                    },
                );
                world.insert(
                    projectile,
                    Sprite {
                        spritesheet: 2,
                        size: sdl2::rect::Point::new(32, 32),
                        src_rect: sdl2::rect::Rect::new(0, 0, 16, 16),
                    },
                );
                world.insert(
                    projectile,
                    Velocity {
                        x: 0.0,
                        y: -weapon.speed as f32,
                    },
                );
                world.insert(projectile, Lifetime { time_left: 0.5 });
                world.insert(
                    projectile,
                    CircleCollider {
                        radius: 16.0,
                        layer: Layers::Bullet | Layers::Enemy
                    },
                );
                world.insert(
                    projectile,
                    Name {
                        name: String::from("Projectile"),
                    },
                    
                );
                world.insert(
                    projectile,
                    CollisionResponse::new()
                );
                world.insert(
                    projectile,
                    Damage{damage:10}       
                );
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
        let (delta, velocity_storage, mut position_storage) = data;
        let delta = delta.0;

        let update_position = |(velocity, position): (& Velocity, &mut Position)| {
            position.x += velocity.x * delta;
            position.y += velocity.y * delta;
        };
        (&velocity_storage, &mut position_storage)
        .join()
        .for_each(update_position);
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
        let (input, controlled, mut velocity_storage, mut weapon_storage) = data;
        let input = &input.0;

        for (velocity, _) in (&mut velocity_storage, &controlled).join() {
            velocity.y = 0.0;
            velocity.x = 0.0;
        }

        if input.get_key(Scancode::Up).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.y = -400.0;
            }
        }

        if input.get_key(Scancode::Down).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.y = 400.0;
            }
        }

        if input.get_key(Scancode::Left).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.x = -400.0;
            }
        }

        if input.get_key(Scancode::Right).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.x = 400.0;
            }
        }

        let should_fire = input.get_key(Scancode::Space).held;
        for (weapon, _) in (&mut weapon_storage, &controlled).join() {
            weapon.wants_to_fire = should_fire;
        }
    }
}
