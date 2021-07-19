use crate::math::{Point3D, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3,
    generation: usize,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3) -> Self {
        Self::with_generation(origin, direction, 0)
    }

    pub fn with_generation(origin: Point3D, direction: Vector3, generation: usize) -> Self {
        Self { origin, direction, generation }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn point_at(&self, distance: f64) -> Point3D {
        (distance * self.direction + Vector3::from(self.origin)).into()
    }

    pub fn reflect(&self, normal: &Vector3, intersection: &Point3D, offset: f64) -> Self {
        Self::with_generation(
            intersection + Point3D::from(normal * offset),
            (self.direction - (2.0 * self.direction.dot(normal) * normal)).normalize(),
            self.generation + 1
        )
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn is_primary(&self) -> bool {
        self.generation() == 0
    }
}
