use specs::prelude::*;
use crate::ecs::components::*;

pub struct CollisionSystem;


fn distance(xa:f32, ya:f32, xb:f32, yb:f32) -> f32
{
    let distance = (xa*xa + ya*ya) - (xb*xb + yb*yb);
    distance.sqrt()
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CircleCollider>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (postions, colliders,names) = data;

       (&postions, &colliders, &names)
        .par_join()
        .for_each(|(pos_a, collider_a, name_a)| {
            for (pos_b, collider_b, name_b) in (&postions, &colliders, &names).join() {
                if collider_a.id == collider_b.id {
                    continue
                }
                let distance = distance(pos_a.x, pos_a.y, pos_b.x, pos_b.y);
                let range = collider_a.radius + collider_b.radius;
                if  distance < range {
                    println!("Collision between {}, {} with distnace {}", name_a.name, name_b.name, distance);
                }   
            }
        });
    }
}