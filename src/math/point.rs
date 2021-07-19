use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::math::Vector3;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point3D(Vector3);

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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
        Point3D::new(other.x, other.y, other.z)
    }
}

impl From<Point3D> for Vector3 {
    fn from(other: Point3D) -> Self {
        Self::new(other.x(), other.y(), other.z())
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
    type Output = Point3D;

    fn add(self, rhs: T) -> Self::Output {
        (self.0 + rhs.as_ref().0).into()
    }
}

impl<T> Add<T> for &Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Point3D;

    fn add(self, rhs: T) -> Self::Output {
        (self.0 + rhs.as_ref().0).into()
    }
}

impl<T> Sub<T> for Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Point3D;

    fn sub(self, rhs: T) -> Self::Output {
        (self.0 - rhs.as_ref().0).into()
    }
}

impl<T> Sub<T> for &Point3D
where
    T: AsRef<Point3D>,
{
    type Output = Point3D;

    fn sub(self, rhs: T) -> Self::Output {
        (self.0 - rhs.as_ref().0).into()
    }
}
