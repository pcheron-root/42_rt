use crate::{Object, Point, Vector};

pub struct LocalIntersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f64,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub object: Object,
    pub t: f64,
    pub point: Point,
    pub normal: Vector,
    pub hit_normal: Vector,
    pub over_point: Point,
    pub reflectv: Vector,
    pub n1: f64,
}

impl Intersection {
    pub fn new(
        object: Object,
        t: f64,
        point: Point,
        normal: Vector,
        hit_normal: Vector,
        over_point: Point,
        reflectv: Vector,
        n1: f64,
    ) -> Self {
        Intersection {
            object,
            t,
            point,
            normal,
            hit_normal,
            over_point,
            reflectv,
            n1,
        }
    }
}
