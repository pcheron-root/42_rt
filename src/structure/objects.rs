use crate::{Matrix, Point, Ray, Shape, Vector, Color};

pub struct Intersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}

pub struct Material {
    pub shininess: f32, // between 10 and 200
    pub specular: f32, // between 0 and 1

    pub color: Color,
    pub ambient: Color,
    pub diffuse: Color,
}

pub struct Object {
    pub material: Material,
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
    pub fn new(shape: Box<dyn Shape>) -> Object {
        Object {
            material: Material{
                specular: 0.9,
                shininess: 100.0,
                color: Color::new([1., 0., 1.]),
                ambient: Color::new([0.1, 0., 0.1]),
                diffuse: Color::new([0.7, 0., 0.7]),
            },
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

    pub fn translate(&mut self, vector: &Vector) {
        self.position = self.position + *vector;

        self.update();
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;

        self.update();
    }

    pub fn scale(&mut self, vector: &Vector) {
        self.scale = *vector;

        self.update();
    }

    fn update(&mut self) {
        let vt = Vector::new([
            self.position.data.x,
            self.position.data.y,
            self.position.data.z,
        ]);

        let translation = Matrix::translation(&vt);
        let rotation = Matrix::rotation(self.pitch, self.yaw, self.roll);
        let scaling = Matrix::scaling(&self.scale);

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
