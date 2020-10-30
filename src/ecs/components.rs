use specs::prelude::*;
use specs_derive::Component;
use crate::vec2::Vec2;

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub position:Vec2,
}

impl Position {
    pub fn new(x:f32, y:f32) -> Self {
        Self {position:Vec2{x, y}}
    }
     
    pub fn x(&self) -> f32 {
        self.position.x
    }
    
    pub fn y(&self) -> f32 {
        self.position.y
    }
}


#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Component, Debug)]
pub struct Lifetime {
    pub time_left: f32,
}

#[derive(Component, Debug)]
pub struct Health {
    pub health: u32,
    pub max_health: u32,
}

impl Health {
    pub fn new(health: u32) -> Health {
        Health {
            health,
            max_health: health,
        }
    }
}

#[derive(Component, Debug)]
pub struct Damage {
    pub damage: u32,
}

impl Health {
    pub fn apply_damage(&mut self, damage: &Damage) {
        if self.health >= damage.damage {
            self.health -= damage.damage;
        } else {
            self.health = 0;
        }
    }
}

#[derive(Debug)]
pub struct KeyboardControlled;

impl Component for KeyboardControlled {
    type Storage = HashMapStorage<Self>;
}

#[derive(Debug)]
pub struct Weapon {
    pub speed: f32,
    pub time_between_shots: f32,
    pub cooldown: f32,
    pub wants_to_fire: bool,
    pub damage: u32,
}

impl Component for Weapon {
    type Storage = HashMapStorage<Self>;
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Velocity {
    pub velocity:Vec2,
}

impl Velocity {
    pub fn new(x:f32, y:f32) -> Self {
        Velocity{ velocity:Vec2{ x,y } }
    }

    pub fn x(&self) -> f32 {
        self.velocity.x
    }

    pub fn y(&self) -> f32 {
        self.velocity.y
    }

    pub fn set_x(&mut self, value:f32) {
        self.velocity.x = value;
    }

    pub fn set_y(&mut self, value:f32) {
        self.velocity.y = value;
    }
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub src_rect: sdl2::rect::Rect,
    /// Size in pixels on screen
    pub size: sdl2::rect::Point,
}
