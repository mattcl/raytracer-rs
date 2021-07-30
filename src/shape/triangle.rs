use crate::{
    material::{Material, TextureCoord, Textured},
    math::{Point2D, Point3D, Vector3, EPSILON},
    ray::Ray,
};

use super::{Intersect, Intersection, Shape, Transformable};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    p0: Point3D,
    p1: Point3D,
    p2: Point3D,
    raw_normal: Vector3,
    material: Material,
}

impl Triangle {
    pub fn new(p0: Point3D, p1: Point3D, p2: Point3D) -> Self {
        Self {
            p0,
            p1,
            p2,
            material: Material::default(),
            raw_normal: (p1 - p0).cross(p2 - p0),
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

impl Intersect for Triangle {
    fn intersect<'a>(&self, ray: &Ray, shape_ref: &'a Shape) -> Option<Intersection<'a>> {
        triangle_intersect(&self.p0, &self.p1, &self.p2, ray)
            .and_then(|(dist, _)| Some(Intersection::new(dist, shape_ref)))
    }

    fn normal_at(&self, _point: &Point3D) -> Option<Vector3> {
        Some(self.raw_normal.normalize())
    }
}

impl Textured for Triangle {
    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        let n = self.raw_normal;
        let denom = n.dot(n);

        let edge1 = self.p2 - self.p1;
        let vp1 = point - self.p1;
        let u = n.dot(edge1.cross(vp1));

        let edge2 = self.p0 - self.p2;
        let vp2 = point - self.p2;
        let v = n.dot(edge2.cross(vp2));

        TextureCoord::new(Point2D::new(u / denom, v / denom), self.material().scale)
    }
}

impl Transformable for Triangle {}

impl From<Triangle> for Shape {
    fn from(t: Triangle) -> Self {
        Shape::Triangle(t)
    }
}


pub fn triangle_intersect(
    p0: &Point3D,
    p1: &Point3D,
    p2: &Point3D,
    ray: &Ray,
) -> Option<(f64, Point2D)> {
    let v0v1 = p1 - p0;
    let v0v2 = p2 - p0;
    pre_calc_traingle_intersect(p0, &v0v1, &v0v2, ray)
}

pub fn pre_calc_traingle_intersect(
    p0: &Point3D,
    v0v1: &Vector3,
    v0v2: &Vector3,
    ray: &Ray,
) -> Option<(f64, Point2D)> {
    let dir = ray.direction();
    let pvec = dir.cross(v0v2);
    let det = v0v1.dot(pvec);

    if det < EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let tvec = ray.origin() - p0;
    let u = tvec.dot(pvec) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(v0v1);
    let v = dir.dot(qvec) * inv_det;

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let d = v0v2.dot(qvec) * inv_det;

    // the triangle is behind us
    if d < 0.0 {
        return None;
    }

    Some((d, Point2D::new(u, v)))
}
