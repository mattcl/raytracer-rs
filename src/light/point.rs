use std::f64::consts::PI;

use crate::{
    color::Color,
    math::{Point3D, Vector3},
};

use super::{Light, Luminous};

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    location: Point3D,
    color: Color,
    intensity: f64,
}

impl PointLight {
    pub fn new(location: Point3D) -> Self {
        PointLight {
            location,
            color: Color::WHITE,
            intensity: 3000.0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity;
        self
    }
}

impl From<PointLight> for Light {
    fn from(p: PointLight) -> Self {
        Light::Point(p)
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self::new(Point3D::new(-2.0, -1.0, -1.0))
    }
}

impl Luminous for PointLight {
    fn color(&self) -> Color {
        self.color
    }

    fn direction_from(&self, point: &Point3D) -> Vector3 {
        Vector3::from(self.location - point).normalize()
    }

    fn distance(&self, point: &Point3D) -> f64 {
        self.location.dist(point)
    }

    fn intensity(&self) -> f64 {
        self.intensity
    }

    fn intensity_at(&self, point: &Point3D) -> f64 {
        let r2 = Vector3::from(self.location - point).norm();

        // TODO: figure out something better to do here - MCL - 2021-07-18
        if r2 < 1e-6_f64 {
            return self.intensity();
        }

        self.intensity() / (4.0 * PI * r2)
    }
}
