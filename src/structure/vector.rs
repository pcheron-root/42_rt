use crate::Dot;
use crate::Point;
use crate::Tuple;

use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub data: Tuple,
}

impl Vector {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(data[0], data[1], data[2], 1.0);
        Self { data }
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
            0.0,
        );

        Self { data: new_tuple }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x - rhs.data.x,
            self.data.y - rhs.data.y,
            self.data.z - rhs.data.z,
            0.0,
        );

        Self { data: new_tuple }
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new([self.data.x * rhs, self.data.y * rhs, self.data.z * rhs])
    }
}

impl Vector {
    fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let len = self.magnitude();
        if len > 0. {
            return Vector::new([self.data.x / len, self.data.y / len, self.data.z / len]);
        }

        self.clone()
    }
}

impl Dot<Point> for Vector {
    fn dot(&self, rhs: Point) -> f32 {
        self.data.x * rhs.data.x + self.data.y * rhs.data.y + self.data.z * rhs.data.z
    }
}

impl Dot<Vector> for Vector {
    fn dot(&self, rhs: Vector) -> f32 {
        self.data.x * rhs.data.x + self.data.y * rhs.data.y + self.data.z * rhs.data.z
    }
}
