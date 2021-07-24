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
        Self {
            origin,
            direction,
            generation,
        }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn point_at(&self, distance: f64) -> Point3D {
        distance * self.direction + self.origin
    }

    pub fn reflect(&self, normal: &Vector3, intersection: &Point3D, offset: f64) -> Self {
        Self::with_generation(
            intersection + normal * offset,
            (self.direction - (2.0 * self.direction.dot(normal) * normal)).normalize(),
            self.generation + 1,
        )
    }

    pub fn refract(
        &self,
        normal: &Vector3,
        intersection: &Point3D,
        offset: f64,
        refractive_index: f64,
    ) -> Option<Self> {
        let mut n = normal.clone();
        let mut eta_i = 1.0;
        let mut eta_t = refractive_index;
        let mut i_dot_n = self.direction.dot(n);
        if i_dot_n < 0.0 {
            i_dot_n = -i_dot_n;
        } else {
            n = -n;
            eta_t = 1.0;
            eta_i = refractive_index;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);

        if k < 0.0 {
            None
        } else {
            Some(Self::with_generation(
                intersection + n * -offset,
                (self.direction() + i_dot_n * n) * eta - n * k.sqrt(),
                self.generation + 1,
            ))
        }
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn is_primary(&self) -> bool {
        self.generation() == 0
    }
}
