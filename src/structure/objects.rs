
use std::f32::EPSILON;

use crate::{Intersection, Matrix, Point, Ray, Shape, Vector, Color};

#[derive(Debug, Clone)]
pub struct Material {
    pub shininess: f32, // between 10 and 200

    pub color: Color,
    pub ambient: f32,  // between 0 and 1
    pub diffuse: f32,  // between 0 and 1
    pub specular: f32, // between 0 and 1

    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new() -> Self {
        let shininess = 200.;
        let color = Color::new(1., 1., 1.);
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let pattern = Some(Pattern::new(Color::new(1.0, 1.0, 1.0), Color::new(0., 0., 0.), true, false, false));
        // let pattern: Option<Pattern> = None;


        Self {
            shininess,
            specular,
            color,
            ambient,
            diffuse,
            pattern,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub material: Material,
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

            // let test = world_normal * EPSILON;
            // eprintln!("wp x:{} y:{} z:{}", world_point.data.x, world_point.data.y, world_point.data.z);
            // eprintln!("x:{} y:{} z:{}", test.data.x, test.data.y, world_normal.data.z);
            let over_point = world_point + world_normal * EPSILON * 64.;
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

#[derive(Debug, Clone)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub x_mod: bool,
    pub y_mod: bool,
    pub z_mod: bool,
}

impl Pattern {
    pub fn new(color_a: Color, color_b: Color, x_mod: bool, y_mod: bool, z_mod: bool) -> Pattern {

        Pattern {
            a : color_a,
            b : color_b,
            x_mod: x_mod,
            y_mod: y_mod,
            z_mod: z_mod,
        }
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        let x;
        if point.x < 0. {
            x = (point.x.abs() + 1.) % 2.;
        
        }
        else {
            x = point.x % 2.0;
        }
        if x < 1. {
            return self.a.clone(); // a verifier 
        }
        self.b.clone()
    }

    pub fn stripe_at_object(&self, obj: &Object, world_point: &Point) -> Color {
        let obj_point = obj.world_to_local.inverse().unwrap() * *world_point; // transform
        let pattern_point =  self.local_to_world.inverse() * obj_point;
        
        self.stripe_at(pattern_point)
    }
}
