use std::f64::consts::PI;

use crate::{
    material::{Material, TextureCoord, Textured},
    math::{Matrix4, Point2D, Point3D, Vector3},
    ray::Ray,
    shape::{Intersect, Shape},
};

use super::{Intersection, Transformable};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Self {
            center,
            radius,
            material: Material::default(),
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersect for Sphere {
    fn intersect<'a>(&self, ray: &Ray, shape_ref: &'a Shape) -> Option<Intersection<'a>> {
        let part = ray.origin() - self.center;
        let b = -(ray.direction().dot(part));
        let del = b * b - part.magnitude().powi(2) + self.radius * self.radius;

        if del < 0.0 {
            return None;
        }

        // There's only one solution and it's actually on the ray
        if del == 0.0 && b >= 0.0 {
            return Some(Intersection::new(b, shape_ref));
        }

        // Otherwise, attempt to find the solution with the smallest positive distance
        let candidates = [b + del.sqrt(), b - del.sqrt()];
        let dist = candidates
            .iter()
            .filter(|dist| **dist >= 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap())?;

        Some(Intersection::new(*dist, shape_ref))
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3> {
        Some((point - self.center).normalize())
    }
}

impl Textured for Sphere {
    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        let v = point - self.center;
        TextureCoord::new(
            Point2D::new(
                (1.0 + v.z().atan2(v.x()) / PI) * 0.5,
                (v.y() / self.radius).acos() / PI,
            ),
            self.material.scale,
        )
    }
}

impl Transformable for Sphere {
    fn transform(&mut self, _matrix: &Matrix4) {
        // nothing for now
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
