use crate::{Point, Vector};

pub struct Camera {
    pub position: Point,
    pub direction: Vector,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Point, direction: Vector, fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            position,
            direction,
            fov,
            near,
            far
        }
    }
}