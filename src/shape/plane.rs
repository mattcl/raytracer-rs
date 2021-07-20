use crate::{
    material::{Material, TextureCoord},
    math::{Point3D, Vector3},
    ray::Ray,
    shape::{Intersect, Shape},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    point: Point3D,
    normal: Vector3,
    material: Material,
    tex_x: Vector3,
    tex_y: Vector3,
}

impl Plane {
    pub fn new(point: Point3D, normal: Vector3) -> Self {
        Self::with_material(point, normal, Material::default())
    }

    pub fn with_material(point: Point3D, normal: Vector3, material: Material) -> Self {
        let mut tex_x = normal.cross(Vector3::K);
        if tex_x.magnitude() == 0.0 {
            tex_x = normal.cross(Vector3::J);
        }

        let tex_x = tex_x.normalize();
        let tex_y = normal.cross(tex_x).normalize();

        Self {
            point,
            normal,
            material,
            tex_x,
            tex_y,
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denominator = ray.direction().dot(-self.normal);
        if denominator < 1e-6_f64 {
            return None;
        }

        let d = (ray.origin() - self.point).dot(self.normal) / denominator;

        if d < 0.0 {
            return None;
        }

        Some(d)
    }

    fn normal_at(&self, _point: &Point3D) -> Option<Vector3> {
        Some(self.normal)
    }

    fn texture_coord(&self, point: &Point3D) -> TextureCoord {
        let v = point - self.point;
        TextureCoord::new(v.dot(self.tex_x), v.dot(self.tex_y), self.material.scale)
    }
}

impl From<Plane> for Shape {
    fn from(p: Plane) -> Self {
        Shape::Plane(p)
    }
}
