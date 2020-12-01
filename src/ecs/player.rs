use crate::ecs::collision::*;
use crate::ecs::components::*;
use specs::prelude::*;

pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Player {}
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Name>);

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::SystemData) {}
}

pub struct PlayerRespawnSystem;

impl<'a> System<'a> for PlayerRespawnSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Player>, Read<'a, LazyUpdate>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_storage, world) = data;

        let mut player_alive = false;

        for _ in (player_storage).join() {
            player_alive = true;
        }

        if !player_alive {
            let new_player = entities.create();

            world.insert(new_player, Position::new(800.0, 800.0));

            world.insert(
                new_player,
                Sprite {
                    spritesheet: crate::assets::PLAYER_SPRITE_ID,
                    size: sdl2::rect::Point::new(64, 64),
                    src_rect: sdl2::rect::Rect::new(0, 0, 128, 128),
                },
            );

            world.insert(new_player, Velocity::default());

            world.insert(
                new_player,
                CircleCollider {
                    radius: 22.0,
                    layer: LayerMask::from_enum(Layers::Player),
                    collides_with: LayerMask::from_enum(Layers::Enemy),
                },
            );

            world.insert(new_player, Health::new(25, on_player_ded));
            world.insert(new_player, Damage::new(5));
            world.insert(new_player, Player::new());
            world.insert(new_player, KeyboardControlled {});

            world.insert(
                new_player,
                crate::ecs::weapon::Weapon::new(1400.0, 0.015, 25),
            );
        }
    }
}

fn on_player_ded(
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
            size: sdl2::rect::Point::new(128, 128),
            src_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
        },
    );
    world.insert(explosion, crate::ecs::animation::Animation::new(30, 4, 4));
    world.insert(explosion, Lifetime { time_left: 0.50 });
}
