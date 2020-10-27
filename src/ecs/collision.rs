use specs::prelude::*;
use specs_derive::Component;

use crate::ecs::components::*;

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
    pub id: u32,
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
    pub fn from_enum_2(tag1: Layers, tag2: Layers) -> Self {
        LayerMask {
            bitmask: (tag1 as usize | tag2 as usize),
        }
    }

    #[allow(dead_code)]
    pub fn from_enum_3(tag1: Layers, tag2: Layers, tag3: Layers) -> Self {
        LayerMask {
            bitmask: (tag1 as usize | tag2 as usize | tag3 as usize),
        }
    }

    #[allow(dead_code)]
    pub fn from_enum_4(tag1: Layers, tag2: Layers, tag3: Layers, tag4: Layers) -> Self {
        LayerMask {
            bitmask: (tag1 as usize | tag2 as usize | tag3 as usize | tag4 as usize),
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
    other: specs::world::Index,
}

impl CollisionResponse {
    pub fn new() -> CollisionResponse {
        CollisionResponse { other: 0 }
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
        ReadStorage<'a, CollisionResponse>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (responses, mut positions): Self::SystemData) {
        self.dirty.clear();

        let events = responses.channel().read(self.reader_id.as_mut().unwrap());

        for event in events {
            if let ComponentEvent::Modified(id) = event {
                self.dirty.add(*id);
            }
        }

        for (response, position, _) in (&responses, &mut positions, &self.dirty).join() {
            println!("Detected collision");
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
        let (entities, 
            postions, 
            colliders, 
            mut responses) = data;

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
                            {
                                let ra = response_a.get_mut_unchecked();
                                ra.other = entity_b.id();
                            }
                        }
                    }
                }
            });
    }
}
