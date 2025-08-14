use crate::utils::lerp;
use crate::{Point, Vector};

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

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
    pub fn new(
        position: Point,
        direction: Vector,
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let direction = direction.normalize();
        let pitch = direction.y.asin().to_degrees();
        let yaw = direction.z.atan2(direction.x).to_degrees();
        Camera {
            target: position.clone(),
            position,
            aspect,
            fov,
            near,
            far,
            pitch: pitch.clamp(-89.0, 89.0), // Clamp pitch to prevent gimbal lock
            yaw,
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
        self.pitch = (self.pitch + angle).clamp(-89.0, 89.0); // Prevent full vertical rotation
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.yaw = (self.yaw + angle) % 360.0;
        // Normalize yaw to [-180, 180] range for consistency
        if self.yaw > 180.0 {
            self.yaw -= 360.0;
        } else if self.yaw < -180.0 {
            self.yaw += 360.0;
        }
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn translate(&mut self, direction: Direction) {
        let up = Vector::new(0.0, 1.0, 0.0);
        let forward = self.direction();
        let right = forward.cross(&up).normalize();

        let movement = match direction {
            Direction::Forward => forward,
            Direction::Backward => -forward,
            Direction::Left => -right,
            Direction::Right => right,
        } * 1.0; // Movement speed

        self.target = self.position + movement;
    }

    pub fn update(&mut self) {
        let factor = 0.75;
        self.position.x = lerp(self.position.x, self.target.x, factor);
        self.position.y = lerp(self.position.y, self.target.y, factor);
        self.position.z = lerp(self.position.z, self.target.z, factor);
    }
}
