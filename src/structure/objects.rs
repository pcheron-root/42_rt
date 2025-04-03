use crate::constants::EPSILON;
use crate::{Intersection, Material, Matrix, Point, Ray, Shape, Transform, Vector};

#[derive(Debug, Clone)]
pub struct Object {
    pub material: Material,
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub scale: Vector,
    pub shape: Shape,

    pub world_to_local: Matrix,
    pub local_to_world: Matrix,
}

impl Object {
    pub fn new(shape: Shape) -> Object {
        Object {
            material: Material::new(),
            position: Point::new(0., 0., 0.),
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
            shape,
            scale: Vector::new(1., 1., 1.),
            world_to_local: Matrix::identity(),
            local_to_world: Matrix::identity(),
        }
    }

    pub fn material(mut self, material: Material) -> Self {
        self.material = material;

        self
    }

    fn update(&mut self) {
        let vt = Vector::new(self.position.x, self.position.y, self.position.z);

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
            let world_normal: Vector = (self.local_to_world.clone() * local_hit.normal).normalize();

            let over_point = world_point + world_normal * EPSILON;

            Some(Intersection::new(
                (*self).clone(),
                local_hit.t,
                world_point,
                world_normal,
                -(ray.direction),
                over_point,
            ))
        } else {
            None
        }
    }
}

impl Transform for Object {
    // move obj
    fn translate(&mut self, vec: Vector) {
        self.position = self.position.clone() + vec;

        self.update();
    }

    fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;

        self.update();
    }

    // grow, shrink the object
    fn scale(&mut self, vec: Vector) {
        self.scale = vec;

        self.update();
    }
}
