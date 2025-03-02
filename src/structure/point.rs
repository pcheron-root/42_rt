use crate::constants::EPSILON;
use crate::Tuple;
use crate::Vector;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub data: Tuple,
}

impl Point {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(data[0], data[1], data[2], 1.0);
        Self { data }
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new([
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
        ])
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new([
            self.data.x - other.data.x,
            self.data.y - other.data.y,
            self.data.z - other.data.z,
        ])
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Point::new([
            self.data.x - other.data.x,
            self.data.y - other.data.y,
            self.data.z - other.data.z,
        ])
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.data.x - other.data.x).abs() < EPSILON
            && (self.data.y - other.data.y).abs() < EPSILON
            && (self.data.z - other.data.z).abs() < EPSILON
    }
}
