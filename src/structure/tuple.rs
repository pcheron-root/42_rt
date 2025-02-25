use crate::constants::EPSILON;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.)
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.)
    }
}

impl Tuple {
    pub fn magnitude(&self) -> f32 {
        if self.is_point() {
            panic!("Cannot calculate magnitude of a point");
        }

        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        if self.is_point() {
            panic!("Cannot normalize of a point");
        }

        let len = self.magnitude();
        if len > 0. {
            return self.clone() / len;
        }

        self.clone()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        if self.is_point() || other.is_point() {
            panic!("Cannot calculate cross product with a point");
        }

        Tuple::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x - rhs.x).abs() < EPSILON
            && (self.y - rhs.y).abs() < EPSILON
            && (self.z - rhs.z).abs() < EPSILON
            && (self.w - rhs.w) == 0.
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_vector() && rhs.is_vector() {
            panic!("Cannot substract a vector by a point");
        }

        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_point() && rhs.is_point() {
            panic!("Cannot add a point to a point");
        } else if self.is_vector() && rhs.is_point() {
            panic!("Cannot add a point to a vector");
        }

        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w,
        }
    }
}
