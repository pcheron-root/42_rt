use crate::{Color, Matrix, Object, Point, Transform, Vector};

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
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub axis: Axis,
    pub blending: bool,

    pub local_to_world: Matrix,
    pub world_to_local: Matrix,
    pub scale: Vector,
    pub position: Point,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Pattern {
    pub fn new(color_a: Color, color_b: Color, axis: Axis, blending: bool) -> Pattern {
        Pattern {
            a: color_a,
            b: color_b,
            axis,
            blending,
            local_to_world: Matrix::identity(),
            world_to_local: Matrix::identity(),
            scale: Vector::new(1., 1., 1.),
            position: Point::new(0., 0., 0.),
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
        }
    }

    pub fn stripe_two_colors(&self, pos1: &f32, pos2: &f32) -> Color {
        if (pos1 * pos1 + pos2 * pos2).sqrt().floor() % 2. == 0. {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn stripe_three_colors(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if self.axis == Axis::XYZ {
            return self.stripe_three_colors(point);
        } else if self.axis == Axis::XY {
            return self.stripe_two_colors(&point.x, &point.y);
        } else if self.axis == Axis::XZ {
            return self.stripe_two_colors(&point.x, &point.z);
        } else if self.axis == Axis::YZ {
            return self.stripe_two_colors(&point.z, &point.y);
        }

        let x = if point.x < 0. {
            (point.x.abs() + 1.) % 2.0
        } else {
            point.x % 2.0
        };

        if x < 1.0 {
            self.a.clone() // a verifier
        } else {
            self.b.clone()
        }
    }

    // should become object method
    pub fn stripe_at_object(&self, obj: &Object, world_point: &Point) -> Color {
        let obj_point = obj.world_to_local.clone() * *world_point;
        let pattern_point = self.world_to_local.clone() * obj_point;

        if self.blending {
            self.pattern_at(&pattern_point)
        } else {
            self.stripe_at(&pattern_point)
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }

    fn update(&mut self) {
        let vt = Vector::new(self.position.x, self.position.y, self.position.z);

        let translation = Matrix::translation(vt);
        let rotation = Matrix::rotation(self.pitch, self.yaw, self.roll);
        let scaling = Matrix::scaling(self.scale.clone());

        self.local_to_world = translation * rotation * scaling;
        self.world_to_local = self.local_to_world.inverse().unwrap();
    }
}

impl Transform for Pattern {
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

    fn scale(&mut self, vec: Vector) {
        self.scale = vec;

        self.update();
    }
}
