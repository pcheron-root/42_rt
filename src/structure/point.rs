use crate::constants::EPSILON;
use crate::Vector;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}
