use std::ops::{Add, Mul, Sub};

use image::{Pixel, Rgba};

use crate::{material::Texture, math::Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color::with_alpha(r, g, b, 1.0)
    }

    pub fn with_alpha(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn mix(&self, other: impl AsRef<Color>, mix_value: f64) -> Color {
        self * (1.0 - mix_value) + other.as_ref() * mix_value
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        &self
    }
}

impl From<Vector3> for Color {
    fn from(v: Vector3) -> Self {
        Color::new(v.x(), v.y(), v.z())
    }
}

impl From<Color> for Vector3 {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b].into()
    }
}

impl From<Color> for Texture {
    fn from(c: Color) -> Self {
        Texture::Color(c)
    }
}

impl<T> Add<T> for Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn add(self, rhs: T) -> Self::Output {
        &self + rhs
    }
}

impl<T> Add<T> for &Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        Color::with_alpha(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a)
    }
}

impl<T> Sub<T> for Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn sub(self, rhs: T) -> Self::Output {
        &self - rhs
    }
}

impl<T> Sub<T> for &Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        Color::with_alpha(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a)
    }
}

impl<T> Mul<T> for Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

impl<T> Mul<T> for &Color
where
    T: AsRef<Color>,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        Color::with_alpha(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::with_alpha(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::with_alpha(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl From<Color> for Rgba<u8> {
    fn from(c: Color) -> Self {
        Rgba::from_channels(
            (c.r * 255.0).clamp(0.0, 255.0) as u8,
            (c.g * 255.0).clamp(0.0, 255.0) as u8,
            (c.b * 255.0).clamp(0.0, 255.0) as u8,
            (c.a * 255.0).clamp(0.0, 255.0) as u8,
        )
    }
}

impl From<Rgba<u8>> for Color {
    fn from(c: Rgba<u8>) -> Self {
        Self::with_alpha(
            (c[0] as f64 / 255.0).clamp(0.0, 1.0),
            (c[1] as f64 / 255.0).clamp(0.0, 1.0),
            (c[2] as f64 / 255.0).clamp(0.0, 1.0),
            (c[3] as f64 / 255.0).clamp(0.0, 1.0),
        )
    }
}
