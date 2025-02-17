use super::tuple::Tuple;
use super::vector::Vector;
use std::ops::{Add, Sub};

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

    fn add(self, other: Self) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x + other.data.x,
            self.data.y + other.data.y,
            self.data.z + other.data.z,
            0.0,
        );

        Self { data: new_tuple }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x + other.data.x,
            self.data.y + other.data.y,
            self.data.z + other.data.z,
            0.0,
        );

        Self { data: new_tuple }
    }
}

pub trait SubPoint {
    fn sub(self, other: Self) -> Vector;
}

impl SubPoint for Point {

    fn sub(self, other: Self) -> Vector {
        let new_tuple = Tuple::new(
            self.data.x - other.data.x,
            self.data.y - other.data.y,
            self.data.z - other.data.z,
            1.0,
        );

        Vector { data: new_tuple }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        let new_tuple = Tuple::new(
            self.data.x - other.data.x,
            self.data.y - other.data.y,
            self.data.z - other.data.z,
            0.0,
        );

        Self { data: new_tuple }
    }
}
