use crate::{Intersection, Matrix, Point, Ray, Shape, Vector};

#[derive(Debug, Clone)]
pub struct Object {
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub scale: Vector,
    pub shape: Shape,

    world_to_local: Matrix,
    local_to_world: Matrix,
}

impl Object {
    pub fn new(shape: Shape) -> Object {
        Object {
            position: Point::new([0., 0., 0.]),
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
            shape,
            scale: Vector::new([1., 1., 1.]),
            world_to_local: Matrix::identity(),
            local_to_world: Matrix::identity(),
        }
    }

    pub fn translate(&mut self, vector: Vector) {
        self.position = self.position.clone() + vector;

        self.update();
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;

        self.update();
    }

    pub fn scale(&mut self, vector: Vector) {
        self.scale = vector;

        self.update();
    }

    fn update(&mut self) {
        let vt = Vector::new([
            self.position.data.x,
            self.position.data.y,
            self.position.data.z,
        ]);

        let translation = Matrix::translation(vt);
        let rotation = Matrix::rotation(self.pitch, self.yaw, self.roll);
        let scaling = Matrix::scaling(self.scale.clone());

        self.local_to_world = translation * rotation * scaling;
        self.world_to_local = self.local_to_world.inverse().unwrap();
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // Transform ray to local space
        let local_ray = self.world_to_local.clone() * ray.clone();

        // Delegate to shape's local-space intersection logic
        if let Some(local_hit) = self.shape.intersect(local_ray) {
            // Transform hit data back to WORLD space
            let world_point: Point = self.local_to_world.clone() * local_hit.point;
            let world_normal: Vector = self.local_to_world.clone() * local_hit.normal;

            Some(Intersection {
                hit_normal: -(ray.direction),
                object: (*self).clone(),
                point: world_point,
                normal: world_normal.normalize(),
                t: local_hit.t,
            })
        } else {
            None
        }
    }
}
