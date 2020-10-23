use specs::prelude::*;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::ecs::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();
    for (pos, sprite) in (&data.0, &data.1).join() {
        let draw_x = pos.x as i32 - (sprite.size.x/2); 
        let draw_y = pos.y as i32 - (sprite.size.y/2); 
        
        let destination = sdl2::rect::Rect::new(draw_x, draw_y, sprite.size.x as u32, sprite.size.y as u32);
        canvas.copy(&textures[sprite.spritesheet], sprite.src_rect, destination)?;
    }

    canvas.present();

    Ok(())
}