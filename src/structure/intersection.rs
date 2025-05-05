use crate::{Object, Point, Vector};

pub struct LocalIntersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub t: f32,
    pub point: Point,
    pub normal: Vector,
    pub eye_normal: Vector,
    pub over_point: Point,
}

impl<'a> Intersection<'a> {
    pub fn new(
        object: &'a Object,
        t: f32,
        point: Point,
        normal: Vector,
        eye_normal: Vector,
        over_point: Point,
    ) -> Self {
        Intersection {
            object,
            t,
            point,
            normal,
            eye_normal,
            over_point,
        }
    }
}
