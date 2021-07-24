use crate::{
    math::{Point3D, Vector3},
    ray::Ray,
};

use super::{Intersect, Intersection, Shape};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    min: Vector3,
    max: Vector3,
}

impl BoundingBox {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
}

impl Intersect for BoundingBox {
    fn intersect<'a>(&self, ray: &Ray, shape_ref: &'a Shape) -> Option<Intersection<'a>> {
        let bounds = [self.min, self.max];
        let invdir = [
            1.0 / ray.direction().x(),
            1.0 / ray.direction().y(),
            1.0 / ray.direction().z(),
        ];
        let sign = [
            (invdir[0] < 0.0) as usize,
            (invdir[1] < 0.0) as usize,
            (invdir[2] < 0.0) as usize,
        ];
        let mut t_min = (bounds[sign[0]].x() - ray.origin().x()) * invdir[0];
        let mut t_max = (bounds[1 - sign[0]].x() - ray.origin().x()) * invdir[0];
        let t_ymin = (bounds[sign[1]].y() - ray.origin().y()) * invdir[1];
        let t_ymax = (bounds[1 - sign[1]].y() - ray.origin().y()) * invdir[1];

        if t_min > t_ymax || t_ymin > t_max {
            return None;
        }

        if t_ymin > t_min {
            t_min = t_ymin;
        }

        if t_ymax < t_max {
            t_max = t_ymax;
        }

        let t_zmin = (bounds[sign[2]].z() - ray.origin().z()) * invdir[2];
        let t_zmax = (bounds[1 - sign[2]].z() - ray.origin().z()) * invdir[2];

        if t_min > t_zmax || t_zmin > t_max {
            return None;
        }

        if t_zmin > t_min {
            t_min = t_zmin;
        }

        if t_zmax < t_max {
            t_max = t_zmax;
        }

        let mut t = t_min;

        if t < 0.0 {
            if t_max < 0.0 {
                return None;
            }
            t = t_max;
        }

        Some(Intersection::new(t, shape_ref))
    }

    fn normal_at(&self, _point: &Point3D) -> Option<Vector3> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    #[test]
    fn yay_flaoting_point_standards() {
        assert_eq!(1.0_f64 / -0.0_f64, NEG_INFINITY);
        assert_eq!(1.0_f64 / 0.0_f64, INFINITY);
    }
}
