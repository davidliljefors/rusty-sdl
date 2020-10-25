use specs::prelude::*;
use crate::ecs::components::*;

pub struct CollisionSystem;


fn circle_collsion(pos_a:&Position, cir_a:&CircleCollider, pos_b:&Position, cir_b:&CircleCollider) -> bool
{
    let dx = pos_a.x - pos_b.x;
    let dy = pos_a.y - pos_b.y;
    let r = cir_a.radius + cir_b.radius;

    dx*dx+ dy*dy < r*r
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
                let impact = circle_collsion(pos_a, collider_a, pos_b, collider_b);
                if  impact {
                    println!("Collision between {}, {}", name_a.name, name_b.name);
                }   
            }
        });
    }
}