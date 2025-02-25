use crate::{Object, Point, Vector};

pub struct LocalHit {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}

#[derive(Clone)]
pub struct Intersection {
    pub object: Object,
    pub t: f32,
    pub point: Point,
    pub normal: Vector,
    pub hit_normal: Vector,
}
