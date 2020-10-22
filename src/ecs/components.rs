use specs::{Component, VecStorage};


#[derive(Debug)]
pub struct Position {
    pub x:f32,
    pub y:f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Controller {
    pub up:bool,
    pub down:bool,
    pub left:bool,
    pub right:bool,
}

impl Component for Controller {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x:f32,
    pub y:f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Name {
    pub name:String,
}

impl Component for Name {
    type Storage = VecStorage<Self>;
}
