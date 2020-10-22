use crate::input;

#[derive(Default)]
pub struct DeltaTime(pub f32);


pub struct Renderer(pub sdl2::render::Canvas<sdl2::video::Window>);

#[derive(Default)]
pub struct InputResource(pub input::Input);