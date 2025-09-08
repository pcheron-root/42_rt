use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn from_rgba(pixel: image::Rgba<u8>) -> Self {
        Color {
            r: pixel[0] as f64 / 255.0,
            g: pixel[1] as f64 / 255.0,
            b: pixel[2] as f64 / 255.0,
        }
    }

    pub fn red(&self) -> f64 {
        self.r
    }

    pub fn green(&self) -> f64 {
        self.g
    }

    pub fn blue(&self) -> f64 {
        self.b
    }

    pub fn to_luminance(self) -> f64 {
        let luminance = 0.299 * self.r + 0.587 * self.g + 0.114 * self.b;
        luminance
    }

}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

// hadamard product
impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b
        )
    }
}

// hadamard product
impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let r: u32 = (self.r.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0) as u32;

        (r << 16) | (g << 8) | (b << 0)
    }
}
