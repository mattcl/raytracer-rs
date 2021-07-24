pub use crate::shape::cube::BoundingBox;
pub use crate::shape::plane::Plane;
pub use crate::shape::sphere::Sphere;
pub use crate::shape::triangle::Triangle;

use crate::{
    material::{Material, TextureCoord, Textured},
    math::{Matrix4, Point3D, Vector3},
    ray::Ray,
};

use self::mesh::TriangleMesh;

pub mod cube;
pub mod mesh;
pub mod plane;
pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub obj: &'a Shape,
    pub distance: f64,
    pub location: Option<Point3D>,
    pub normal: Option<Vector3>,
    pub tex_coord: Option<TextureCoord>,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, obj: &Shape) -> Intersection {
        Intersection {
            distance,
            obj,
            location: None,
            normal: None,
            tex_coord: None,
        }
    }

    pub fn location(mut self, point: Point3D) -> Self {
        self.location = Some(point);
        self
    }

    pub fn normal(mut self, normal: Vector3) -> Self {
        self.normal = Some(normal);
        self
    }

    pub fn texture_coord(mut self, tex: TextureCoord) -> Self {
        self.tex_coord = Some(tex);
        self
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub trait Transformable {
    fn transform(&mut self, matrix: &Matrix4);

    fn world_to_object(&self) -> Option<&Matrix4> {
        None
    }

    fn object_to_world(&self) -> Option<&Matrix4> {
        None
    }

    fn transform_to_world<T>(&self, t: T) -> T
    where
        T: From<Vector3>,
        Vector3: From<T>,
    {
        match self.object_to_world() {
            Some(m) => T::from(m * Vector3::from(t)),
            None => t,
        }
    }

    fn transform_to_object<T>(&self, t: T) -> T
    where
        T: From<Vector3>,
        Vector3: From<T>,
    {
        match self.world_to_object() {
            Some(m) => T::from(m * Vector3::from(t)),
            None => t,
        }
    }

    fn transform_normal_to_world(&self, normal: Vector3) -> Vector3 {
        match self.world_to_object() {
            Some(m) => (m.transpose() * normal).into(),
            None => normal,
        }
    }

    fn transform_normal_to_object(&self, normal: Vector3) -> Vector3 {
        match self.object_to_world() {
            Some(m) => (m.transpose() * normal).into(),
            None => normal,
        }
    }
}

pub trait Intersect {
    // so this is a litle ugly, but since the intersection needs to ref the parent
    // shape, we need to pass a reference down
    fn intersect<'a>(&self, ray: &Ray, shape_ref: &'a Shape) -> Option<Intersection<'a>>;
    fn normal_at(&self, point: &Point3D) -> Option<Vector3>;
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
    TriangleMesh(TriangleMesh),
}

impl Shape {
    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => s.material(),
            Shape::Plane(s) => s.material(),
            Shape::Triangle(s) => s.material(),
            Shape::TriangleMesh(s) => s.material(),
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersection<'a>> {
        match self {
            Shape::Sphere(ref s) => s.intersect(ray, self),
            Shape::Plane(ref s) => s.intersect(ray, self),
            Shape::Triangle(ref s) => s.intersect(ray, self),
            Shape::TriangleMesh(ref s) => s.intersect(ray, self),
        }
    }

    pub fn normal_at(&self, point: &Point3D) -> Option<Vector3> {
        match self {
            Shape::Sphere(ref s) => s.normal_at(point),
            Shape::Plane(ref s) => s.normal_at(point),
            Shape::Triangle(ref s) => s.normal_at(point),
            Shape::TriangleMesh(ref s) => s.normal_at(point),
        }
    }

    pub fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        match self {
            Shape::Sphere(ref s) => s.texture_coord(point),
            Shape::Plane(ref s) => s.texture_coord(point),
            Shape::Triangle(ref s) => s.texture_coord(point),
            Shape::TriangleMesh(ref s) => s.texture_coord(point),
        }
    }
}

impl Transformable for Shape {
    fn world_to_object(&self) -> Option<&Matrix4> {
        match self {
            Shape::Sphere(s) => s.world_to_object(),
            Shape::Plane(s) => s.world_to_object(),
            Shape::Triangle(s) => s.world_to_object(),
            Shape::TriangleMesh(s) => s.world_to_object(),
        }
    }

    fn object_to_world(&self) -> Option<&Matrix4> {
        match self {
            Shape::Sphere(s) => s.object_to_world(),
            Shape::Plane(s) => s.object_to_world(),
            Shape::Triangle(s) => s.object_to_world(),
            Shape::TriangleMesh(s) => s.object_to_world(),
        }
    }

    fn transform(&mut self, matrix: &Matrix4) {
        match self {
            Shape::Sphere(s) => s.transform(matrix),
            Shape::Plane(s) => s.transform(matrix),
            Shape::Triangle(s) => s.transform(matrix),
            Shape::TriangleMesh(s) => s.transform(matrix),
        }
    }
}
