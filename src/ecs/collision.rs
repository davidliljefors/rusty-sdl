use specs::prelude::*;
use specs_derive::Component;

use crate::{ecs::components::*, vec2::Vec2};

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
    pub layer: LayerMask,
    pub collides_with: LayerMask,
}

#[derive(Copy, Clone)]
pub enum Layers {
    Player = 0x01,
    Enemy = 0x02,
}

impl std::ops::BitOr for Layers {
    type Output = LayerMask;
    fn bitor(self, rhs: Self) -> LayerMask {
        let result = self as usize | rhs as usize;
        LayerMask::new(result)
    }
}

#[derive(Debug, Copy, Clone)]
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

fn circle_collsion(
    pos_a: &Position,
    cir_a: &CircleCollider,
    pos_b: &Position,
    cir_b: &CircleCollider,
) -> bool {
    let distance = Vec2::distance(pos_a.position, pos_b.position);
    distance < cir_a.radius + cir_b.radius
}

pub struct CollisionSystem {
    collisions: std::sync::Arc<std::sync::Mutex<Vec<(specs::Entity, specs::Entity)>>>,
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {
            collisions: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CircleCollider>,
        ReadStorage<'a, Damage>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, postions, colliders, damage_storage, mut health_storage) = data;
        (&entities, &postions, &colliders)
            .par_join()
            .for_each(|(entity_a, pos_a, collider_a)| {
                for (entity_b, pos_b, collider_b) in (&entities, &postions, &colliders).join() {
                    if entity_a == entity_b {
                        continue;
                    }

                    if collider_a.collides_with.any(&collider_b.layer) {
                        let impact = circle_collsion(pos_a, collider_a, pos_b, collider_b);
                        if impact {
                            self.collisions.lock().unwrap().push((entity_a, entity_b));
                        }
                    }
                }
            });

        for (a, b) in self.collisions.lock().unwrap().iter() {
            let target_health = health_storage.get_mut(*b);
            let source_damage = damage_storage.get(*a);
            let damage_position = postions.get(*b);

            if let Some(target_health) = target_health {
                if let Some(source_damage) = source_damage {
                    if let Some(damage_position) = damage_position {
                        target_health.apply_damage(source_damage, damage_position.position);
                    }
                }
            }
        }

        self.collisions.lock().unwrap().clear();
    }
}
