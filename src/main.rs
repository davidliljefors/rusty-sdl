
mod engine;
mod input;
mod ecs {
    pub mod components;
    pub mod resources;
    pub mod systems;
}


pub fn main() {
    let engine = engine::Engine::new();

    engine.run();
}
