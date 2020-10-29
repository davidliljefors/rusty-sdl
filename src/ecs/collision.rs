use specs::prelude::*;
use specs_derive::Component;

use crate::ecs::components::*;
use crate::ecs::animation::*;

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
    pub layer: LayerMask,
}

#[derive(Copy, Clone)]
pub enum Layers {
    Player = 0x01,
    Enemy = 0x02,
    Bullet = 0x04,
}

impl std::ops::BitOr for Layers {
    type Output = LayerMask;

    fn bitor(self, rhs: Self) -> LayerMask {
        let result = self as usize | rhs as usize;
        LayerMask::new(result)
    }
}

#[derive(Debug)]
pub struct LayerMask {
    bitmask: usize,
}

impl LayerMask {
    #[allow(dead_code)]
    pub fn new(bitmask: usize) -> Self {
        LayerMask { bitmask }
    }

    #[allow(dead_code)]
    pub fn from_enum(tag: Layers) -> Self {
        LayerMask {
            bitmask: tag as usize,
        }
    }

    #[allow(dead_code)]
    fn any(&self, other: &LayerMask) -> bool {
        self.bitmask & other.bitmask != 0x00
    }

    #[allow(dead_code)]
    fn is_superset(&self, other: &LayerMask) -> bool {
        let both = self.bitmask & other.bitmask;
        both == other.bitmask
    }

    #[allow(dead_code)]
    fn all(&self, other: &LayerMask) -> bool {
        self.bitmask == other.bitmask
    }
}

pub struct CollisionResponse {
    other: Option<specs::world::Entity>,
}

impl CollisionResponse {
    pub fn new() -> CollisionResponse {
        CollisionResponse { other: None }
    }
}

impl Component for CollisionResponse {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Default)]
pub struct ResponseSystem {
    pub dirty: BitSet,
    pub reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ResponseSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CollisionResponse>,
        ReadStorage<'a, Damage>,
        ReadStorage<'a, Projectile>,
        WriteStorage<'a, Health>,
        Read<'a, LazyUpdate>
    );

    fn run(
        &mut self,
        (entities, position_storage, response_storage, damage_storage, projectile_storage, mut health_storage, world): Self::SystemData,
    ) {
        self.dirty.clear();

        let events = response_storage.channel().read(self.reader_id.as_mut().unwrap());

        for event in events {
            if let ComponentEvent::Modified(id) = event {
                self.dirty.add(*id);
            }
        }

        for (response, damage, _) in (&response_storage, &damage_storage, &self.dirty).join() {
            if let Some(target) = response.other {
                let target_health = health_storage.get_mut(target);
                if let Some(target_health) = target_health {
                    target_health.apply_damage(damage);
                }
            }
        }

        for (entity, position, _, _) in
            (&entities, &position_storage, &projectile_storage, &self.dirty).join()
        {
            let explosion = entities.create();

            world.insert(
                explosion,
                Position {
                    x: position.x,
                    y: position.y,
                }
            );
            world.insert(
                explosion,
                Sprite {
                    spritesheet: 3,
                    size: sdl2::rect::Point::new(32, 32),
                    src_rect: sdl2::rect::Rect::new(0, 0, 64, 64),
                }
            );
            world.insert(
                explosion,
                Animation::new(30, 4, 4)
            );
            world.insert(
                explosion,
                Lifetime {time_left:0.50}
            );

            entities.delete(entity).expect("Error delete projectile on collision");
        }
    }

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.reader_id = Some(WriteStorage::<CollisionResponse>::fetch(&res).register_reader());
    }
}

fn circle_collsion(
    pos_a: &Position,
    cir_a: &CircleCollider,
    pos_b: &Position,
    cir_b: &CircleCollider,
) -> bool {
    let dx = pos_a.x - pos_b.x;
    let dy = pos_a.y - pos_b.y;
    let r = cir_a.radius + cir_b.radius;

    (dx * dx) + (dy * dy) < (r * r)
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CircleCollider>,
        WriteStorage<'a, CollisionResponse>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, postions, colliders, mut responses) = data;
        (
            &entities,
            &postions,
            &colliders,
            &mut responses.par_restrict_mut(),
        )
            .par_join()
            .for_each(|(entity_a, pos_a, collider_a, mut response_a)| {
                for (entity_b, pos_b, collider_b) in (&entities, &postions, &colliders).join() {
                    if entity_a == entity_b {
                        continue;
                    }

                    if collider_a.layer.any(&collider_b.layer) {
                        let impact = circle_collsion(pos_a, collider_a, pos_b, collider_b);
                        if impact {
                            let ra = response_a.get_mut_unchecked();
                            ra.other = Some(entity_b);
                        }
                    }
                }
            });
    }
}
