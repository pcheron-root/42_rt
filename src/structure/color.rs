use crate::Tuple;

use std::convert::Into;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub data: Tuple,
}

impl Color {
    pub fn new(data: [f32; 3]) -> Self {
        let data = Tuple::new(
            data[0],
            data[1],
            data[2],
            2.0,
        );

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

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let r: u32 = (self.data.x.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.data.y.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.data.z.clamp(0.0, 1.0) * 255.0) as u32;

        (r << 16) | (g << 8) | (b << 0)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = ((value >> 16) & 0xFF) as f32 / 255.0;
        let g = ((value >> 8) & 0xFF) as f32 / 255.0;
        let b = ((value >> 0) & 0xFF) as f32 / 255.0;

        Color::new([r, g, b])
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new([
            (self.data.x + rhs.data.x),
            (self.data.y + rhs.data.y),
            (self.data.z + rhs.data.z),
        ])
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new([
            (self.data.x - rhs.data.x),
            (self.data.y - rhs.data.y),
            (self.data.z - rhs.data.z),
        ])
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new([
            (self.data.x * rhs),
            (self.data.y * rhs),
            (self.data.z * rhs),
        ])
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new([
            (self.data.x * rhs.data.x),
            (self.data.y * rhs.data.y),
            (self.data.z * rhs.data.z),
        ])
    }
}
