use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::math::Vector3;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3D(Vector3);

impl Point3D {
    pub fn new<T, M, N>(x: T, y: M, z: N) -> Self
    where
        T: Into<f64> + Copy,
        M: Into<f64> + Copy,
        N: Into<f64> + Copy,
    {
        Self(Vector3::new(x, y, z))
    }

    pub fn dist(&self, other: impl AsRef<Point3D>) -> f64 {
        let other = other.as_ref();
        ((self.0.x - other.0.x).powi(2)
            + (self.0.y - other.0.y).powi(2)
            + (self.0.z - other.0.z).powi(2))
        .sqrt()
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn z(&self) -> f64 {
        self.0.z
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

impl AsRef<Point3D> for Point3D {
    fn as_ref(&self) -> &Point3D {
        &self
    }
}

impl<T> Add<T> for Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Vector3;

    fn add(self, rhs: T) -> Self::Output {
        self.0 + rhs.as_ref().0
    }
}

impl<T> Add<T> for &Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Vector3;

    fn add(self, rhs: T) -> Self::Output {
        self.0 + rhs.as_ref().0
    }
}

impl<T> Sub<T> for Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Vector3;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs.as_ref().0
    }
}

impl<T> Sub<T> for &Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Vector3;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs.as_ref().0
    }
}

impl Add<Vector3> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<&Vector3> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: &Vector3) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<Vector3> for &Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<&Vector3> for &Point3D {
    type Output = Point3D;

    fn add(self, rhs: &Vector3) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<Point3D> for Vector3 {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        (self + rhs.0).into()
    }
}

impl Add<&Point3D> for Vector3 {
    type Output = Point3D;

    fn add(self, rhs: &Point3D) -> Self::Output {
        (self + rhs.0).into()
    }
}

impl Add<Point3D> for &Vector3 {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        (self + rhs.0).into()
    }
}

impl Add<&Point3D> for &Vector3 {
    type Output = Point3D;

    fn add(self, rhs: &Point3D) -> Self::Output {
        (self + rhs.0).into()
    }
}

impl Sub<Vector3> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3) -> Self::Output {
        (self.0 - rhs).into()
    }
}

impl Sub<&Vector3> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        (self.0 - rhs).into()
    }
}

impl Sub<Vector3> for &Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3) -> Self::Output {
        (self.0 - rhs).into()
    }
}

impl Sub<&Vector3> for &Point3D {
    type Output = Point3D;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        (self.0 - rhs).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_between_points_makes_a_vector() {
        let a = Point3D::new(2, 3, 4);
        let b = Point3D::new(5, 6, 7);
        let c = Vector3::new(7, 9, 11);

        assert_eq!(a + b, c);
        assert_eq!(a + &b, c);
        assert_eq!(&a + b, c);
        assert_eq!(&a + &b, c);
    }

    #[test]
    fn subtraction_between_points_makes_a_vector() {
        let a = Point3D::new(2, 3, 4);
        let b = Point3D::new(5, 6, 7);
        let c = Vector3::new(-3, -3, -3);

        assert_eq!(a - b, c);
        assert_eq!(a - &b, c);
        assert_eq!(&a - b, c);
        assert_eq!(&a - &b, c);
    }

    #[test]
    fn addition_between_point_and_vector_makes_a_point() {
        let a = Point3D::new(12, 3, 4);
        let b = Vector3::new(15, 6, 7);
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
