use sdl2::keyboard::Scancode;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::ecs::weapon::*;

pub struct HealthSystem;

impl<'a> System<'a> for HealthSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Health>, Read<'a, LazyUpdate>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut health_storage, world) = data;

        for (entity, health) in (&entities, &mut health_storage).join() {
            for damage_event in health.damage_events.iter() {
                let DamageEvent::DamageTaken(amount, pos) = damage_event;

                if amount >= &health.health {
                    health.health = 0;
                    (health.on_death)(*pos, &entities, &world);
                    entities.delete(entity).expect("error deleeting entity")
                // enemy died!
                } else {
                    health.health -= amount;
                }
            }
            health.damage_events.clear();
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

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta_time, velocity_storage, mut position_storage) = data;
        let delta_time = delta_time.0;

        let update_position = |(velocity, position): (&Velocity, &mut Position)| {
            position.position += velocity.velocity * delta_time;
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
            velocity.set_y(0.0);
            velocity.set_x(0.0);
        }

        if input.get_key(Scancode::Up).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.set_y(-400.0);
            }
        }

        if input.get_key(Scancode::Down).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.set_y(400.0);
            }
        }

        if input.get_key(Scancode::Left).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.set_x(-400.0);
            }
        }

        if input.get_key(Scancode::Right).held {
            for (velocity, _) in (&mut velocity_storage, &controlled).join() {
                velocity.set_x(400.0);
            }
        }

        let should_fire = input.get_key(Scancode::Space).held;

        if should_fire {
            for (weapon, _) in (&mut weapon_storage, &controlled).join() {
                weapon.command = WeaponFireCommand::FireOnce;
            }
        } else {
            for (weapon, _) in (&mut weapon_storage, &controlled).join() {
                weapon.command = WeaponFireCommand::Waiting;
            }
        }
    }
}
