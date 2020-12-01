use crate::ecs::resources::DeltaTime;
use rand::Rng;
use specs::prelude::*;

use crate::ecs::components::*;
use crate::ecs::player::*;
use crate::ecs::weapon::*;
use crate::vec2::Vec2;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn next(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn to_vec(&self) -> Vec2 {
        match *self {
            Direction::Right => Vec2::right(),
            Direction::Down => Vec2::down(),
            Direction::Left => Vec2::left(),
            Direction::Up => Vec2::up(),
        }
    }
}

enum EnemyMoveState {
    Idle,
    Moving(Direction, Vec2),
}

#[allow(unused)]
pub enum AttackConfig {
    Constant(f32, u32),
    Random(f32, f32, u32),
}

pub struct Enemy {
    speed: f32,
    move_state: EnemyMoveState,
    attack_config: AttackConfig,
    attack_cooldown: f32,
}

impl Enemy {
    pub fn new(speed: f32, attack_config: AttackConfig) -> Enemy {
        let attack_cooldown: f32;
        match attack_config {
            AttackConfig::Constant(cooldown, _) => {
                attack_cooldown = cooldown;
            }
            AttackConfig::Random(_, max_cooldown, _) => {
                attack_cooldown = rand::thread_rng().gen_range(0.0, max_cooldown);
            }
        }
        Enemy {
            speed,
            move_state: EnemyMoveState::Idle,
            attack_config,
            attack_cooldown,
        }
    }

    fn fire(&mut self, weapon: &mut Weapon) {
        match self.attack_config {
            AttackConfig::Constant(cooldown, amount) => {
                weapon.command = WeaponFireCommand::FireAmount(amount);
                self.attack_cooldown = cooldown;
            }
            AttackConfig::Random(min_cooldown, max_cooldown, amount) => {
                weapon.command = WeaponFireCommand::FireAmount(amount);
                self.attack_cooldown = rand::thread_rng().gen_range(min_cooldown, max_cooldown);
            }
        }
    }
}

impl Component for Enemy {
    type Storage = HashMapStorage<Self>;
}

pub struct EnemySystem;

impl<'a> System<'a> for EnemySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Weapon>,
    );

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::SystemData) {
        let (
            delta_time,
            position_storage,
            player_storage,
            mut enemy_storage,
            mut velocity_storage,
            mut weapon_storage,
        ) = data;
        let delta_time = delta_time.0;

        for (enemy_pos, mut enemy_compoent, mut enemy_velocity, weapon) in (
            &position_storage,
            &mut enemy_storage,
            &mut velocity_storage,
            &mut weapon_storage,
        )
            .join()
        {
            enemy_compoent.attack_cooldown -= delta_time;

            if enemy_compoent.attack_cooldown <= 0.0 {
                enemy_compoent.fire(weapon);
            }

            match enemy_compoent.move_state {
                EnemyMoveState::Idle => {
                    let target = enemy_pos.position + Vec2::right() * 100.0;
                    enemy_compoent.move_state = EnemyMoveState::Moving(Direction::Right, target);
                }

                EnemyMoveState::Moving(direction, target) => {
                    let dot_prod = Vec2::dot(&(target - enemy_pos.position), &direction.to_vec());

                    let new_vel = direction.to_vec() * enemy_compoent.speed;
                    enemy_velocity.velocity = new_vel;

                    if dot_prod < 0.0 {
                        let new_direction = direction.next();
                        let new_target = target + new_direction.to_vec() * 100.0;
                        enemy_compoent.move_state =
                            EnemyMoveState::Moving(new_direction, new_target);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct EnemySpawnerSystem {
    wave_counter: u32,
}

impl EnemySpawnerSystem {
    pub fn default() -> Self {
        EnemySpawnerSystem { wave_counter: 0 }
    }
}

impl<'a> System<'a> for EnemySpawnerSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Enemy>, Read<'a, LazyUpdate>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, enemy_storage, world) = data;

        let mut enemy_count = 0;

        for _ in (enemy_storage).join() {
            enemy_count += 1;
        }

        let mut enemy_spawned_emount = 0;
        if enemy_count == 0 {
            self.wave_counter += 1;

            for x in 0..(16 * self.wave_counter) {
                for y in 0..(4 * self.wave_counter) {
                    let new_enemy = entities.create();
                    enemy_spawned_emount += 1;
                    world.insert(
                        new_enemy,
                        Position::new(
                            (100 + x * 80 / self.wave_counter) as f32,
                            (100 + y * 80 / self.wave_counter) as f32,
                        ),
                    );

                    world.insert(
                        new_enemy,
                        Sprite {
                            spritesheet: crate::assets::BOSS_SPRITE_ID,
                            size: sdl2::rect::Point::new(
                                64 / self.wave_counter as i32,
                                64 / self.wave_counter as i32,
                            ),
                            src_rect: sdl2::rect::Rect::new(0, 0, 128, 128),
                        },
                    );

                    world.insert(new_enemy, Velocity::default());

                    world.insert(
                        new_enemy,
                        crate::ecs::collision::CircleCollider {
                            radius: 32.0,
                            layer: crate::ecs::collision::LayerMask::from_enum(
                                crate::ecs::collision::Layers::Enemy,
                            ),
                            collides_with: crate::ecs::collision::LayerMask::from_enum(
                                crate::ecs::collision::Layers::Player,
                            ),
                        },
                    );

                    world.insert(new_enemy, Health::new(25, on_enemy_ded));

                    world.insert(new_enemy, Damage::new(5));

                    world.insert(
                        new_enemy,
                        Enemy::new(100.0, AttackConfig::Random(5.0, 15.0, 3)),
                    );

                    world.insert(new_enemy, Weapon::new(-800.0, 0.5, 5));
                }
            }
            println!("Spawned {:?} new enemies", enemy_spawned_emount);
        }
    }
}

fn on_enemy_ded(position: Vec2, entities: &specs::Entities, world: &specs::LazyUpdate) {
    let explosion = entities.create();

    world.insert(
        explosion,
        Position {
            position: Vec2::randomize(position, 10.0),
        },
    );
    world.insert(
        explosion,
        Sprite {
            spritesheet: crate::assets::EXPLOSION_SPRITE_ID,
            size: sdl2::rect::Point::new(64, 64),
            src_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
        },
    );
    world.insert(explosion, crate::ecs::animation::Animation::new(30, 4, 4));
    world.insert(explosion, Lifetime { time_left: 0.50 });
}
