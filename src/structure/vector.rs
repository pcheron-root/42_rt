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

    pub fn magnitude(self) -> f32 {
        (self.data.x * self.data.x + self.data.y * self.data.y + self.data.z * self.data.z).sqrt()
    }
}

// -----------------------------------------------------------------
// Operator overloading
// -----------------------------------------------------------------

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

