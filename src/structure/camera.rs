use crate::utils::lerp;
use crate::{Point, Vector};

pub struct Camera {
    pub target: Point,
    pub position: Point,
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,

    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn new(position: Point, aspect: f32, fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            target: position.clone(),
            position,
            aspect,
            fov,
            near,
            far,

            pitch: 0.,
            yaw: 270.,
        }
    }

    pub fn direction(&self) -> Vector {
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        Vector::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize()
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.pitch = (self.pitch + 360. + angle) % 360.;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.yaw = (self.yaw + 360. + angle) % 360.;
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn translate(&mut self, vector: Vector) {
        self.target = self.position.clone() + vector;
    }

    pub fn update(&mut self) {
        let factor = 0.75;

        self.position.x = lerp(self.position.x, self.target.x, factor);
        self.position.y = lerp(self.position.y, self.target.y, factor);
        self.position.z = lerp(self.position.z, self.target.z, factor);
    }
}
