use specs::prelude::*;
use specs_derive::Component;

use crate::ecs::components::*;
use crate::ecs::resources::*;

#[derive(Component, Debug)]
pub struct Animation {
    pub time_per_frame:f32,
    pub sheet_columns:u32,
    pub sheet_rows:u32,

    internal_counter:f32,
    internal_index:u32,
}

impl Animation {
    pub fn new(fps:u32, sheet_columns:u32, sheet_rows:u32) -> Animation {
        Animation { time_per_frame:(1.0 / fps as f32) , sheet_columns, sheet_rows, internal_index:0, internal_counter:0.0}
    }

    fn update(&mut self, delta_time:f32, sprite_src:&mut sdl2::rect::Rect ) {
        self.internal_counter += delta_time;

        if self.internal_counter > self.time_per_frame {
            self.internal_counter -= self.time_per_frame;
            self.internal_index += 1;

            if self.internal_index > ( self.sheet_columns * self.sheet_rows ) {
                self.internal_index = 0;
            }

            let row = self.internal_index % self.sheet_columns;
            let column = self.internal_index / self.sheet_columns;

            sprite_src.set_x((row * sprite_src.height()) as i32);
            sprite_src.set_y((column * sprite_src.width()) as i32);
        }
    }
}

pub struct AnimationSystem;

impl <'a> System<'a> for AnimationSystem {
    type SystemData = (
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Animation>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let(mut sprite_storage, mut animation_storage, delta_time) = data;
        let delta_time = delta_time.0;

        (&mut sprite_storage, &mut animation_storage)
        .par_join()
        .for_each( |(sprite, animation)| {
            animation.update( delta_time, &mut sprite.src_rect )
        });
    }
}