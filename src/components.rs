use nalgebra::{Point2, Vector2};
use specs::prelude::*;

#[derive(Debug, Clone)]
pub struct Mass(pub f32);

impl Mass {
    pub fn new(m: f32) -> Self {
        Mass(m)
    }
}

impl Component for Mass {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Vel(pub Vector2<f32>);

impl Vel {
    pub fn new(x: f32, y: f32) -> Self {
        Vel(Vector2::new(x, y))
    }
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Pos(pub Point2<f32>);

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos(Point2::new(x, y))
    }
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}
