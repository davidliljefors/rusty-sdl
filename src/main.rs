mod assets;
mod engine;
mod input;
mod vec2;
mod ecs {
    pub mod animation;
    pub mod collision;
    pub mod components;
    pub mod enemy;
    pub mod player;
    pub mod renderer;
    pub mod resources;
    pub mod systems;
    pub mod weapon;
}

pub fn main() {
    let engine = engine::Engine::new(1600, 900);

    engine.run();
}
