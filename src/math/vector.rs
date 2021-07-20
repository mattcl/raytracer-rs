use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const I: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const J: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const K: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new<T, M, N>(x: T, y: M, z: N) -> Self
    where
        T: Into<f64> + Copy,
        M: Into<f64> + Copy,
        N: Into<f64> + Copy,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn dot(&self, other: impl AsRef<Self>) -> f64 {
        let other = other.as_ref();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: impl AsRef<Self>) -> Self {
        let Self {
            x: bx,
            y: by,
            z: bz,
        } = self;
        let Self {
            x: cx,
            y: cy,
            z: cz,
        } = other.as_ref();

        Self {
            x: by * cz - bz * cy,
            y: bz * cx - bx * cz,
            z: bx * cy - by * cx,
        }
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    fn zip<F>(&self, other: impl AsRef<Self>, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let other = other.as_ref();
        Self {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
        }
    }

    fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl AsRef<Vector3> for Vector3 {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl<T> Add<T> for Vector3
where
    T: AsRef<Vector3>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.zip(rhs, |a, b| a + b)
    }
}

impl<T> Add<T> for &Vector3
where
    T: AsRef<Vector3>,
{
    type Output = Vector3;

    fn add(self, rhs: T) -> Self::Output {
        self.zip(rhs, |a, b| a + b)
    }
}

impl<T> Sub<T> for Vector3
where
    T: AsRef<Vector3>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.zip(rhs, |a, b| a - b)
    }
}

impl<T> Sub<T> for &Vector3
where
    T: AsRef<Vector3>,
{
    type Output = Vector3;

    fn sub(self, rhs: T) -> Self::Output {
        self.zip(rhs, |a, b| a - b)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs.map(|a| a * self)
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        rhs.map(|a| a * self)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negation() {
        let a = Vector3::new(1.0, 10.0, 100.0);
        let b = Vector3::new(-1.0, -10.0, -100.0);

        assert_eq!(-a, b);
        assert_eq!(-&a, b);
    }

    #[test]
    fn addition() {
        let a = Vector3::new(1.0, 10.0, 100.0);
        let b = Vector3::new(2.0, 20.0, 200.0);
        let c = Vector3::new(3.0, 30.0, 300.0);

        assert_eq!(a + b, c);
        assert_eq!(a + &b, c);
        assert_eq!(&a + b, c);
        assert_eq!(&a + &b, c);
    }

    #[test]
    fn subtraction() {
        let a = Vector3::new(1.0, 10.0, 100.0);
        let b = Vector3::new(2.0, 20.0, 200.0);
        let c = Vector3::new(-1.0, -10.0, -100.0);

        assert_eq!(a - b, c);
        assert_eq!(a - &b, c);
        assert_eq!(&a - b, c);
        assert_eq!(&a - &b, c);
    }

    #[test]
    fn multiplication() {
        let a = Vector3::new(1.0, 10.0, 100.0);
        let b = 2.0;
        let c = Vector3::new(2.0, 20.0, 200.0);

        assert_eq!(a * b, c);
        assert_eq!(&a * b, c);
        assert_eq!(b * a, c);
        assert_eq!(b * &a, c);
    }

    #[test]
    fn division() {
        let a = Vector3::new(1.0, 10.0, 100.0);
        let b = 2.0;
        let c = Vector3::new(0.5, 5.0, 50.0);

        assert_eq!(a / b, c);
        assert_eq!(&a / b, c);
    }

    #[test]
    fn dotproduct() {
        let a = Vector3::new(9.0, 2.0, 7.0);
        let b = Vector3::new(4.0, 8.0, 10.0);
        let c = 122.0;

        assert_eq!(a.dot(b), c);
        assert_eq!(a.dot(&b), c);
        assert_eq!(b.dot(a), c);
        assert_eq!(b.dot(&a), c);
    }

    #[test]
    fn crossproduct() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        let c = Vector3::new(-3.0, 6.0, -3.0);

        assert_eq!(a.cross(b), c);
        assert_eq!(a.cross(&b), c);
    }

    #[test]
    fn magnitude() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();

        assert_eq!(a.magnitude(), b);
    }

    #[test]
    fn normalize() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = a / a.magnitude();

        assert_eq!(a.normalize(), b);

        assert_eq!(Vector3::K.normalize(), Vector3::K);
    }

    #[test]
    fn ergonomics() {
        // can consruct from anything that satisfies Into<Float> + Copy
        let a = Vector3::new(2, 3, 4);
        let b = Vector3::new(2, 3.0, 4_f32);
        let c = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(a, c);
        assert_eq!(b, c);
    }
}
