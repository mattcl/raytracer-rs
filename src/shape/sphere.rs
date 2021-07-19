use std::f64::consts::PI;

use crate::{
    material::{Material, TextureCoord},
    math::{Point3D, Vector3},
    ray::Ray,
    shape::{Intersect, Shape},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Self::with_material(center, radius, Material::default())
    }

    pub fn with_material(center: Point3D, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let part: Vector3 = (ray.origin() - self.center).into();
        let b = -(ray.direction().dot(part));
        let del = b * b - part.magnitude().powi(2) + self.radius * self.radius;

        if del < 0.0 {
            return None;
        }

        // There's only one solution and it's actually on the ray
        if del == 0.0 && b >= 0.0 {
            return Some(b);
        }

        // Otherwise, attempt to find the solution with the smallest positive distance
        let candidates = [b + del.sqrt(), b - del.sqrt()];
        let dist = candidates
            .iter()
            .filter(|dist| **dist >= 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap())?;

        Some(*dist)
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3> {
        Some(Vector3::from(point - self.center).normalize())
    }

    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        let v = Vector3::from(point - self.center);
        TextureCoord::new(
            (1.0 + v.z.atan2(v.x) / PI) * 0.5,
            (v.y / self.radius).acos() / PI,
            self.material.scale,
        )
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
