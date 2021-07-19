use std::f64::INFINITY;

use crate::{
    color::Color,
    math::{Point3D, Vector3},
};

use super::{Light, Luminous};

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalLight {
    direction: Vector3,
    color: Color,
    intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vector3, color: Color, intensity: f64) -> Self {
        Self {
            direction,
            color,
            intensity,
        }
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self::new(Vector3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0)
    }
}

impl From<DirectionalLight> for Light {
    fn from(l: DirectionalLight) -> Self {
        Light::Directional(l)
    }
}

impl Luminous for DirectionalLight {
    fn color(&self) -> Color {
        self.color
    }

    fn direction_from(&self, _point: &Point3D) -> Vector3 {
        -self.direction
    }

    fn distance(&self, _point: &Point3D) -> f64 {
        INFINITY
    }

    fn intensity(&self) -> f64 {
        self.intensity
    }

    fn intensity_at(&self, _point: &Point3D) -> f64 {
        self.intensity()
    }
}
