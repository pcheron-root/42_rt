use crate::{Color, Matrix, Point, Texturable, Transformable, Vector};

#[derive(Debug, Clone, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ,
}

#[derive(Debug, Clone)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub axis: Axis,
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub scale: Vector,
    pub local_to_world: Matrix,
    pub world_to_local: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color, axis: Axis) -> Self {
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
            axis,
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

impl Stripe {
    fn stripe_one_axis(&self, axis: f32) -> Color {
        let x = if axis < 0.0 {
            (axis.abs() + 1.) % 2.0
        } else {
            axis % 2.0
        };

        if x < 1.0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    fn stripe_two_axis(&self, axis1: f32, axis2: f32) -> Color {
        if (axis1 * axis1 + axis2 * axis2).sqrt().floor() % 2. == 0. {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    fn stripe_three_axis(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

impl Texturable for Stripe {
    fn color_at(&self, point: &Point) -> Color {
        let point = self.world_to_local.clone() * point.clone();

        if self.axis == Axis::XYZ {
            self.stripe_three_axis(&point)
        } else if self.axis == Axis::XY {
            self.stripe_two_axis(point.x, point.y)
        } else if self.axis == Axis::XZ {
            self.stripe_two_axis(point.x, point.z)
        } else if self.axis == Axis::YZ {
            self.stripe_two_axis(point.z, point.y)
        } else {
            self.stripe_one_axis(point.x)
        }
    }
}

impl Transformable for Stripe {
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
