pub use crate::light::directional::DirectionalLight;
pub use crate::light::point::PointLight;

use crate::{
    color::Color,
    math::{Point3D, Vector3},
};

mod directional;
mod point;

pub trait Luminous: Into<Light> {
    fn color(&self) -> Color;
    fn direction_from(&self, point: &Point3D) -> Vector3;
    fn distance(&self, point: &Point3D) -> f64;
    fn intensity(&self) -> f64;
    fn intensity_at(&self, point: &Point3D) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match self {
            Light::Directional(light) => light.color(),
            Light::Point(light) => light.color(),
        }
    }

    pub fn direction_from(&self, point: &Point3D) -> Vector3 {
        match self {
            Light::Directional(light) => light.direction_from(point),
            Light::Point(light) => light.direction_from(point),
        }
    }

    pub fn distance(&self, point: &Point3D) -> f64 {
        match self {
            Light::Directional(light) => light.distance(point),
            Light::Point(light) => light.distance(point),
        }
    }

    pub fn intensity_at(&self, point: &Point3D) -> f64 {
        match self {
            Light::Directional(light) => light.intensity_at(point),
            Light::Point(light) => light.intensity_at(point),
        }
    }
}
