pub use matrix::Matrix4;
pub use point::{Point2D, Point3D};
pub use transform::Transform;
pub use vector::{Vector, Vector2, Vector3, Vector4};

mod matrix;
mod point;
mod transform;
mod vector;

pub const EPSILON: f64 = 1e-10_f64;
