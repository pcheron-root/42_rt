// use crate::traits::type_trait::TypeTrait;
use std::ops::{Neg, Mul, Div};
// use std::ops::{Add, Sub};


pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

// pas teste
// impl Sub for Tuple {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         let new_tuple = Tuple::new(
//             self.x - other.x,
//             self.y - other.y,
//             self.z - other.z,
//             self.w,
//         );

//         new_tuple
//     }
// }


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
