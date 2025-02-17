use crate::Dot;
use crate::Tuple;
use crate::Vector;
use std::ops::{Add, Div, Sub};

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

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new([
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
        ])
    }
}

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
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new([
            self.data.x - rhs.data.x,
            self.data.y - rhs.data.y,
            self.data.z - rhs.data.z,
        ])
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Point::new([self.data.x / rhs, self.data.y / rhs, self.data.z / rhs])
    }
}

impl Dot<Point> for Point {
    fn dot(&self, rhs: Point) -> f32 {
        self.data.x * rhs.data.x + self.data.y * rhs.data.y + self.data.z * rhs.data.z
    }
}

impl Dot<Vector> for Point {
    fn dot(&self, rhs: Vector) -> f32 {
        self.data.x * rhs.data.x + self.data.y * rhs.data.y + self.data.z * rhs.data.z
    }
}
