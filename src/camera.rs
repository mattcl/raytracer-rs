use crate::math::{Point3D, Vector3};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3D,
    forward: Vector3,
    right: Vector3,
    up: Vector3,
    fov_deg: f64,
}

impl Camera {
    pub fn new(origin: Point3D, look: &Point3D, fov_deg: f64) -> Self {
        let mut cam = Self {
            origin,
            forward: Vector3::ZERO,
            right: Vector3::ZERO,
            up: Vector3::ZERO,
            fov_deg,
        };

        cam.look_at(look);
        cam
    }

    pub fn look_at(&mut self, point: &Point3D) {
        self.forward = Vector3::from(point - self.origin).normalize();
        self.right = Vector3::J.cross(self.forward);
        self.up = self.forward.cross(self.right);
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn forward(&self) -> &Vector3 {
        &self.forward
    }

    pub fn right(&self) -> &Vector3 {
        &self.right
    }

    pub fn up(&self) -> &Vector3 {
        &self.up
    }

    pub fn fov_radians(&self) -> f64 {
        self.fov_deg.to_radians()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3D::new(0.0, 0.0, -20.0),
            &Point3D::new(0.0, 0.0, 0.0),
            70.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let c = Camera::default();
        assert_eq!(c.forward, Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(c.up, Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(c.right, Vector3::new(1.0, 0.0, 0.0));
    }
}
