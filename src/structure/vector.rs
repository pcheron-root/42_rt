use super::tuple::Tuple;
use std::ops::{Add, Sub};

pub struct Vector {
    pub data: Tuple,
}

impl Vector {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(data[0], data[1], data[2], 1.0);
        Self { data }
    }

    pub fn magnitude(&self) -> f32 {
        (self.data.x * self.data.x + self.data.y * self.data.y + self.data.z * self.data.z).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Vector {
                data : Tuple {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
                }
            };
        }
        Vector {
            data : Tuple {
            x: self.data.x / mag,
            y: self.data.y / mag,
            z: self.data.z / mag,
            w: 1.0,
            }
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.data.x * other.data.x +
        self.data.y * other.data.y +
        self.data.z * other.data.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            data: Tuple {
                x: self.data.y * other.data.z - self.data.z * other.data.y,
                y: self.data.z * other.data.x - self.data.x * other.data.z,
                z: self.data.x * other.data.y - self.data.y * other.data.x,
                w: 1.0,
            }
        }
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.data.x - other.data.x).abs() < f32::EPSILON &&
        (self.data.y - other.data.y).abs() < f32::EPSILON &&
        (self.data.z - other.data.z).abs() < f32::EPSILON &&
        (self.data.w - other.data.w).abs() < f32::EPSILON
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x + other.data.x,
            self.data.y + other.data.y,
            self.data.z + other.data.z,
            1.0,
        );

        Self { data: new_tuple }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x - other.data.x,
            self.data.y - other.data.y,
            self.data.z - other.data.z,
            1.0,
        );

        Self { data: new_tuple }
    }
}

