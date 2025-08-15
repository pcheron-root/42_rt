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
    pub aspect: f64,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Camera {
    pub fn new(
        position: Point,
        direction: Vector,
        aspect: f64,
        fov: f64,
        near: f64,
        far: f64,
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
            pitch: pitch.clamp(-89.9, 89.9),
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

    pub fn rotate_x(&mut self, angle: f64) {
        self.pitch = (self.pitch + angle).clamp(-89.9, 89.9);
    }

    pub fn rotate_y(&mut self, angle: f64) {
        self.yaw = (self.yaw + angle) % 360.0;
        if self.yaw > 180.0 {
            self.yaw -= 360.0;
        } else if self.yaw < -180.0 {
            self.yaw += 360.0;
        }
    }

    pub fn resize(&mut self, aspect: f64) {
        self.aspect = aspect;
    }

    pub fn translate(&mut self, direction: Direction) {
        let up = Vector::new(0.0, 1.0, 0.0);
        let forward = self.direction();
        let right = forward.cross(&up).normalize();

        let speed = 1.0;
        let movement = match direction {
            Direction::Forward => forward,
            Direction::Backward => -forward,
            Direction::Left => -right,
            Direction::Right => right,
        } * speed;

        self.target = self.position + movement;
    }

    pub fn update(&mut self) {
        let factor = 0.75;
        self.position.x = lerp(self.position.x, self.target.x, factor);
        self.position.y = lerp(self.position.y, self.target.y, factor);
        self.position.z = lerp(self.position.z, self.target.z, factor);
    }
}
