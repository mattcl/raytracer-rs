pub use crate::shape::plane::Plane;
pub use crate::shape::sphere::Sphere;
pub use crate::shape::triangle::Triangle;

use crate::{
    material::{Material, TextureCoord},
    math::{Point3D, Vector3},
    ray::Ray,
};

pub mod plane;
pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub obj: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, obj: &Shape) -> Intersection {
        Intersection { distance, obj }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub trait Intersect: Into<Shape> {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal_at(&self, point: &Point3D) -> Option<Vector3>;
    fn texture_coord(&self, point: &Point3D) -> TextureCoord;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceData {
    pub texture_x: f64,
    pub texture_y: f64,
    pub normal: Vector3,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(sphere) => sphere.material(),
            Shape::Plane(plane) => plane.material(),
        }
    }
}

impl Intersect for Shape {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Shape::Sphere(ref sphere) => sphere.intersect(ray),
            Shape::Plane(ref plane) => plane.intersect(ray),
        }
    }

    fn normal_at(&self, point: &Point3D) -> Option<Vector3> {
        match self {
            Shape::Sphere(ref sphere) => sphere.normal_at(point),
            Shape::Plane(ref plane) => plane.normal_at(point),
        }
    }

    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        match self {
            Shape::Sphere(ref sphere) => sphere.texture_coord(point),
            Shape::Plane(ref plane) => plane.texture_coord(point),
        }
    }
}
