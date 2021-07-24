pub use matrix::Matrix4;
pub use point::{Point2D, Point3D};
pub use vector::{Vector2, Vector3};

mod matrix;
mod point;
mod vector;

pub const EPSILON: f64 = 1e-10_f64;
