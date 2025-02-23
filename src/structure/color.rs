
// use crate::constants::EPSILON;
use crate::Tuple;
// use crate::Vector;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub data: Tuple,
}

impl Color {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(data[0], data[1], data[2], 2.0);
        Self { data }
    }

    pub fn red(&self) -> f32 {
        self.data.x
    }

    pub fn green(&self) -> f32 {
        self.data.y
    }

    pub fn blue(&self) -> f32 {
        self.data.z
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new([
            self.data.x + rhs.data.x,
            self.data.y + rhs.data.y,
            self.data.z + rhs.data.z,
        ])
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new([
            self.data.x - rhs.data.x,
            self.data.y - rhs.data.y,
            self.data.z - rhs.data.z,
        ])
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new([self.data.x * rhs, self.data.y * rhs, self.data.z * rhs])
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new([
            self.data.x * rhs.data.x,
            self.data.y * rhs.data.y,
            self.data.z * rhs.data.z,
        ])
    }
}

pub fn hadamard_product(c1: &Color, c2: &Color) -> Color{
    Color::new([
        c1.data.x * c2.data.x,
        c1.data.y * c2.data.y,
        c1.data.z * c2.data.z,
    ])
}
