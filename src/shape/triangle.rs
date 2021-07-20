use crate::{
    material::{Material, TextureCoord},
    math::{Point3D, Vector3, EPSILON},
    ray::Ray,
};

use super::{Intersect, Shape};

pub struct Vertex(pub Vector3);

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
        Self::with_material(p0, p1, p2, Material::default())
    }

    pub fn with_material(p0: Point3D, p1: Point3D, p2: Point3D, material: Material) -> Self {
        Self {
            p0,
            p1,
            p2,
            material,
            raw_normal: (p1 - p0).cross(p2 - p0),
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let v0v1 = self.p1 - self.p0;
        let v0v2 = self.p2 - self.p0;
        let pvec = ray.direction().cross(v0v2);
        let det = v0v1.dot(pvec);

        if det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin() - self.p0;
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction().dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        Some(v0v2.dot(qvec) * inv_det)
    }

    fn normal_at(&self, _point: &Point3D) -> Option<Vector3> {
        Some(self.raw_normal.normalize())
    }

    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        let n = self.raw_normal;
        let denom = n.dot(n);

        let edge1 = self.p2 - self.p1;
        let vp1 = point - self.p1;
        let u = n.dot(edge1.cross(vp1));

        let edge2 = self.p0 - self.p2;
        let vp2 = point - self.p2;
        let v = n.dot(edge2.cross(vp2));

        TextureCoord::new(u / denom, v / denom, self.material().scale)
    }
}

impl From<Triangle> for Shape {
    fn from(t: Triangle) -> Self {
        Shape::Triangle(t)
    }
}
