use crate::{Color, Matrix, Point, Texturable, Transformable, Vector};

#[derive(Debug, Clone)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub scale: Vector,
    pub local_to_world: Matrix,
    pub world_to_local: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        let position = Point::new(0.0, 0.0, 0.0);
        let pitch = 0.0;
        let yaw = 0.0;
        let roll = 0.0;
        let scale = Vector::new(1.0, 1.0, 1.0);

        let local_to_world = Matrix::identity();
        let world_to_local = Matrix::identity();

        Self {
            a,
            b,
            position,
            pitch,
            yaw,
            roll,
            scale,
            local_to_world,
            world_to_local,
        }
    }
}

impl Texturable for Gradient {
    fn color_at(&self, point: &Point) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

impl Transformable for Gradient {
    fn translate(&mut self, vec: Vector) {
        self.position = self.position.clone() + vec;

        self.update_transformation();
    }

    fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;

        self.update_transformation();
    }

    fn scale(&mut self, vec: Vector) {
        self.scale = vec;

        self.update_transformation();
    }

    fn update_transformation(&mut self) {
        let vt = Vector::new(self.position.x, self.position.y, self.position.z);

        let translation = Matrix::translation(vt);
        let rotation = Matrix::rotation(self.pitch, self.yaw, self.roll);
        let scaling = Matrix::scaling(self.scale.clone());

        self.local_to_world = translation * rotation * scaling;
        self.world_to_local = self.local_to_world.inverse().unwrap();
    }
}
