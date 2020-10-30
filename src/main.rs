
mod engine;
mod input;
mod vec2;
mod ecs {
    pub mod components;
    pub mod resources;
    pub mod systems;
    pub mod renderer;
    pub mod collision;
    pub mod animation;
}


pub fn main() {
    let engine = engine::Engine::new(1920, 1080);

    engine.run();
}
