use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{
    error::{RTError, Result},
    math::{Vector2, Vector3},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3D(Vector3);

impl Point3D {
    pub fn new<T, M, N>(x: T, y: M, z: N) -> Self
    where
        T: Into<f64> + Copy,
        M: Into<f64> + Copy,
        N: Into<f64> + Copy,
    {
        Self(Vector3::new([x.into(), y.into(), z.into()]))
    }

    pub fn dist(&self, other: impl AsRef<Point3D>) -> f64 {
        let other = other.as_ref();
        ((self.0.x() - other.0.x()).powi(2)
            + (self.0.y() - other.0.y()).powi(2)
            + (self.0.z() - other.0.z()).powi(2))
        .sqrt()
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }
}

impl Index<usize> for Point3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Point3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl From<Vector3> for Point3D {
    fn from(other: Vector3) -> Self {
        Point3D(other)
    }
}

impl From<&Vector3> for Point3D {
    fn from(other: &Vector3) -> Self {
        Point3D(other.clone())
    }
}

impl From<Point3D> for Vector3 {
    fn from(other: Point3D) -> Self {
        other.0
    }
}

impl From<&Point3D> for Vector3 {
    fn from(other: &Point3D) -> Self {
        other.0.clone()
    }
}

impl From<[f64; 3]> for Point3D {
    fn from(a: [f64; 3]) -> Self {
        Vector3::from(a).into()
    }
}

impl TryFrom<Vec<f64>> for Point3D {
    type Error = RTError;

    fn try_from(value: Vec<f64>) -> Result<Self> {
        Ok(Point3D(value.try_into()?))
    }
}

impl AsRef<Point3D> for Point3D {
    fn as_ref(&self) -> &Point3D {
        &self
    }
}

impl_op_ex!(+ |a: &Point3D, b: &Point3D| -> Vector3 { a.0 + b.0 } );
impl_op_ex!(-|a: &Point3D, b: &Point3D| -> Vector3 { a.0 - b.0 });
impl_op_ex_commutative!(+ |a: &Vector3, b: &Point3D| -> Point3D { (a + b.0).into() } );
impl_op_ex!(-|a: &Point3D, b: &Vector3| -> Vector3 { a.0 - b });

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point2D(Vector2);

impl Point2D {
    pub fn new<T, M>(x: T, y: M) -> Self
    where
        T: Into<f64> + Copy,
        M: Into<f64> + Copy,
    {
        Self([x.into(), y.into()].into())
    }

    pub fn dist(&self, other: impl AsRef<Point2D>) -> f64 {
        let other = other.as_ref();
        ((self.0.x() - other.0.x()).powi(2) + (self.0.y() - other.0.y()).powi(2)).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl From<Vector2> for Point2D {
    fn from(other: Vector2) -> Self {
        Point2D(other)
    }
}

impl From<&Vector2> for Point2D {
    fn from(other: &Vector2) -> Self {
        Point2D(other.clone())
    }
}

impl From<Point2D> for Vector2 {
    fn from(other: Point2D) -> Self {
        other.0
    }
}

impl From<&Point2D> for Vector2 {
    fn from(other: &Point2D) -> Self {
        other.0.clone()
    }
}

impl TryFrom<Vec<f64>> for Point2D {
    type Error = RTError;

    fn try_from(value: Vec<f64>) -> Result<Self> {
        Ok(Point2D(value.try_into()?))
    }
}

impl AsRef<Point2D> for Point2D {
    fn as_ref(&self) -> &Point2D {
        &self
    }
}

impl_op_ex!(+ |a: &Point2D, b: &Point2D| -> Vector2 { a.0 + b.0 } );
impl_op_ex!(-|a: &Point2D, b: &Point2D| -> Vector2 { a.0 - b.0 });
impl_op_ex_commutative!(+ |a: &Vector2, b: &Point2D| -> Point2D { (a + b.0).into() } );
impl_op_ex!(-|a: &Point2D, b: &Vector2| -> Vector2 { a.0 - b });

#[cfg(test)]
mod tests {
    mod point3 {
        use super::super::*;

        #[test]
        fn addition_between_points_makes_a_vector() {
            let a = Point3D::new(2, 3, 4);
            let b = Point3D::new(5, 6, 7);
            let c = Vector3::new([7.0, 9.0, 11.0]);

            assert_eq!(a + b, c);
            assert_eq!(a + &b, c);
            assert_eq!(&a + b, c);
            assert_eq!(&a + &b, c);
        }

        #[test]
        fn subtraction_between_points_makes_a_vector() {
            let a = Point3D::new(2, 3, 4);
            let b = Point3D::new(5, 6, 7);
            let c = Vector3::new([-3.0, -3.0, -3.0]);

            assert_eq!(a - b, c);
            assert_eq!(a - &b, c);
            assert_eq!(&a - b, c);
            assert_eq!(&a - &b, c);
        }

        #[test]
        fn addition_between_point_and_vector_makes_a_point() {
            let a = Point3D::new(12, 3, 4);
            let b = Vector3::new([15.0, 6.0, 7.0]);
            let c = Point3D::new(27, 9, 11);

            assert_eq!(a + b, c);
            assert_eq!(a + &b, c);
            assert_eq!(&a + b, c);
            assert_eq!(&a + &b, c);

            assert_eq!(b + a, c);
            assert_eq!(b + &a, c);
            assert_eq!(&b + a, c);
            assert_eq!(&b + &a, c);
        }
    }

    mod point2 {
        use super::super::*;

        #[test]
        fn addition_between_points_makes_a_vector() {
            let a = Point2D::new(2, 3);
            let b = Point2D::new(5, 6);
            let c = Vector2::new([7.0, 9.0]);

            assert_eq!(a + b, c);
            assert_eq!(a + &b, c);
            assert_eq!(&a + b, c);
            assert_eq!(&a + &b, c);
        }

        #[test]
        fn subtraction_between_points_makes_a_vector() {
            let a = Point2D::new(2, 3);
            let b = Point2D::new(5, 6);
            let c = Vector2::new([-3.0, -3.0]);

            assert_eq!(a - b, c);
            assert_eq!(a - &b, c);
            assert_eq!(&a - b, c);
            assert_eq!(&a - &b, c);
        }

        #[test]
        fn addition_between_point_and_vector_makes_a_point() {
            let a = Point2D::new(12, 3);
            let b = Vector2::new([15.0, 6.0]);
            let c = Point2D::new(27, 9);

            assert_eq!(a + b, c);
            assert_eq!(a + &b, c);
            assert_eq!(&a + b, c);
            assert_eq!(&a + &b, c);

            assert_eq!(b + a, c);
            assert_eq!(b + &a, c);
            assert_eq!(&b + a, c);
            assert_eq!(&b + &a, c);
        }
    }
}
