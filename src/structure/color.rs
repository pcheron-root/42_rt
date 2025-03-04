use std::convert::Into;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

use crate::constants::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn red(&self) -> f32 {
        self.r
    }

    pub fn green(&self) -> f32 {
        self.g
    }

    pub fn blue(&self) -> f32 {
        self.b
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

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = ((value >> 16) & 0xFF) as f32 / 255.0;
        let g = ((value >> 8) & 0xFF) as f32 / 255.0;
        let b = ((value >> 0) & 0xFF) as f32 / 255.0;

        Color::new(r, g, b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r - other.r).abs() < EPSILON
            && (self.g - other.g).abs() < EPSILON
            && (self.b - other.b).abs() < EPSILON
    }
}