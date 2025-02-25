use crate::utils::lerp;
use crate::{Point, Vector};

pub struct Camera {
    pub target: Point,
    pub position: Point,
    pub direction: Vector,
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        position: Point,
        direction: Vector,
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera {
            target: position.clone(),
            position,
            direction,
            aspect,
            fov,
            near,
            far,
        }
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn translate(&mut self, vector: Vector) {
        self.target = self.position.clone() + vector;
    }

    pub fn update(&mut self) {
        let factor = 0.75;

        self.position.data.x = lerp(self.position.data.x, self.target.data.x, factor);
        self.position.data.y = lerp(self.position.data.y, self.target.data.y, factor);
        self.position.data.z = lerp(self.position.data.z, self.target.data.z, factor);
    }
}
