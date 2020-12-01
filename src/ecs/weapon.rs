use crate::ecs::collision::*;
use crate::ecs::components::*;
use crate::ecs::resources::*;
use specs::prelude::*;

pub enum WeaponFireCommand {
    Waiting,
    FireOnce,
    FireAmount(u32),
}

pub struct Weapon {
    pub speed: f32,
    pub time_between_shots: f32,
    pub cooldown: f32,
    pub command: WeaponFireCommand,
    pub damage: u32,
}

impl Weapon {
    pub fn new(speed: f32, time_between_shots: f32, damage: u32) -> Weapon {
        Weapon {
            speed,
            time_between_shots,
            cooldown: 0.0,
            command: WeaponFireCommand::Waiting,
            damage,
        }
    }
}

impl Component for Weapon {
    type Storage = HashMapStorage<Self>;
}

pub struct WeaponSystem;

impl<'a> System<'a> for WeaponSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CircleCollider>,
        WriteStorage<'a, Weapon>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, delta, position, collider, mut weapon, world): Self::SystemData) {
        let delta = delta.0;

        let handle_weapon = |(_entity, position, collider, weapon): (
            Entity,
            &Position,
            &CircleCollider,
            &mut Weapon,
        )| {
            weapon.cooldown -= delta;

            if weapon.cooldown <= 0.0 {
                match weapon.command {
                    WeaponFireCommand::FireOnce => {
                        spawn_bullet(position, collider, weapon, &entities, &world);
                        weapon.command = WeaponFireCommand::Waiting;
                    }
                    WeaponFireCommand::FireAmount(amount) => {
                        spawn_bullet(position, collider, weapon, &entities, &world);
                        let new_amount = amount - 1;
                        if new_amount > 0 {
                            weapon.command = WeaponFireCommand::FireAmount(new_amount);
                        } else {
                            weapon.command = WeaponFireCommand::Waiting;
                        }
                    }
                    _ => {}
                }
            }
        };

        (&entities, &position, &collider, &mut weapon)
            .join()
            .for_each(handle_weapon);
    }
}

fn spawn_bullet(
    position: &Position,
    collider: &CircleCollider,
    weapon: &mut Weapon,
    entities: &specs::Entities,
    world: &specs::LazyUpdate,
) {
    let projectile = entities.create();
    weapon.cooldown = weapon.time_between_shots;

    world.insert(projectile, Projectile {});
    world.insert(projectile, *position);
    world.insert(
        projectile,
        Sprite {
            spritesheet: crate::assets::BULLET_SPRITE_ID,
            size: sdl2::rect::Point::new(crate::assets::BULLET_SIZE, crate::assets::BULLET_SIZE),
            src_rect: sdl2::rect::Rect::new(0, 0, 16, 16),
        },
    );
    world.insert(projectile, Velocity::new(0.0, -weapon.speed as f32));
    world.insert(projectile, Lifetime { time_left: 1.0 });
    world.insert(
        projectile,
        CircleCollider {
            radius: 16.0,
            layer: collider.layer,
            collides_with: collider.collides_with,
        },
    );
    world.insert(projectile, Health::new(1, on_bullet_dead));
    world.insert(
        projectile,
        Damage {
            damage: weapon.damage,
        },
    );
}

fn on_bullet_dead(
    position: crate::vec2::Vec2,
    entities: &specs::Entities,
    world: &specs::LazyUpdate,
) {
    let explosion = entities.create();

    world.insert(
        explosion,
        Position {
            position: crate::vec2::Vec2::randomize(position, 10.0),
        },
    );
    world.insert(
        explosion,
        Sprite {
            spritesheet: crate::assets::EXPLOSION_SPRITE_ID,
            size: sdl2::rect::Point::new(32, 32),
            src_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
        },
    );
    world.insert(explosion, crate::ecs::animation::Animation::new(30, 4, 4));
    world.insert(explosion, Lifetime { time_left: 0.50 });
}
