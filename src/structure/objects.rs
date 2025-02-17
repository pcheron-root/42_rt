use crate::{Matrix, Point, Ray, Shape, Vector};

pub struct Intersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}

pub struct Object {
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub scale: Vector,
    pub shape: Box<dyn Shape>,

    world_to_local: Matrix,
    local_to_world: Matrix,
}

impl Object {
    pub fn new(shape: Box<dyn Shape>, position: Point) -> Object {
        Object {
            position,
            shape,
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
            scale: Vector::new([1., 1., 1.]),
            world_to_local: Matrix::identity(),
            local_to_world: Matrix::identity(),
        }
    }

    pub fn update_matrices(&mut self) {
        // let translation = Matrix::translation(self.position.data.x, self.position.y, self.position.z);
        // let rotation = Matrix::euler_angles(self.pitch, self.yaw, self.roll);
        // let scaling = Matrix::scaling(self.scale.x, self.scale.y, self.scale.z);

        let translation = Matrix::identity();
        let rotation = Matrix::identity();
        let scaling = Matrix::identity();

        self.local_to_world = translation * rotation * scaling;
        self.world_to_local = self.local_to_world.inverse().unwrap();
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Transform ray to local space
        let ray = self.world_to_local * *ray;

        // Delegate to shape's local-space intersection logic
        if let Some(local_hit) = self.shape.intersect(&ray) {
            // Transform hit data back to WORLD space
            let world_point = self.local_to_world * local_hit.point;
            let world_normal = self.local_to_world * local_hit.normal;

            Some(Intersection {
                point: world_point,
                normal: world_normal.normalize(),
                t: local_hit.t,
            })
        } else {
            None
        }
    }
}
