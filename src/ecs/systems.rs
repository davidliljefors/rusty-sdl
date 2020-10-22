use specs::{Read, ReadStorage, System, WriteStorage, Join};
use sdl2::keyboard::Scancode;

use crate::ecs::components::*;
use crate::ecs::resources::*;

pub struct PositionPrinterSystem;

impl<'a> System<'a> for PositionPrinterSystem {
    type SystemData = (ReadStorage<'a, Position>,  ReadStorage<'a, Name> );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, name) = data;   

        for (pos, name) in (&pos, &name).join() {
            println!("{:?} is at {:?}", &name.name, &pos);
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
        let (delta, vel, mut pos) = data;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, InputResource>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, mut velocity) = data;
        let input = &input.0;

        for velocity in (&mut velocity).join() {
            velocity.y = 0.0;
            velocity.x = 0.0;
        }
        
        if input.get_key(Scancode::Up).held
        {
            for velocity in (&mut velocity).join() {
                velocity.y = -1.0;
            }
        }

        if input.get_key(Scancode::Down).held
        {
            for velocity in (&mut velocity).join() {
                velocity.y = 1.0;
            }
        }

        if input.get_key(Scancode::Left).held
        {
            for velocity in (&mut velocity).join() {
                velocity.x = -1.0;
            }
        }

        if input.get_key(Scancode::Right).held
        {
            for velocity in (&mut velocity).join() {
                velocity.x = 1.0;
            }
        }
    }
}
