use crate::constants::EPSILON;
use crate::Tuple;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct Vector {
    pub data: Tuple,
}

impl Vector {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(data[0], data[1], data[2], 0.0);

        Self { data }
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let len = self.magnitude();
        if len > 0. {
            return Vector::new([self.data.x / len, self.data.y / len, self.data.z / len]);
        }

        self.clone()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.data.x * other.data.x + self.data.y * other.data.y + self.data.z * other.data.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector::new([
            self.data.y * other.data.z - self.data.z * other.data.y,
            self.data.z * other.data.x - self.data.x * other.data.z,
            self.data.x * other.data.y - self.data.y * other.data.x,
        ])
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.data.x - other.data.x).abs() < EPSILON
            && (self.data.y - other.data.y).abs() < EPSILON
            && (self.data.z - other.data.z).abs() < EPSILON
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new([
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
        ])
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new([
            self.data.x - rhs.data.x,
            self.data.y - rhs.data.y,
            self.data.z - rhs.data.z,
        ])
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new([self.data.x * rhs, self.data.y * rhs, self.data.z * rhs])
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new([self.data.x / rhs, self.data.y / rhs, self.data.z / rhs])
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new([
            -self.data.x,
            -self.data.y,
            -self.data.z,
        ])
    }
}