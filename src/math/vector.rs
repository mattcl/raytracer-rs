use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    ops::{Index, IndexMut, Neg},
};

use crate::error::{RTError, Result};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector<const N: usize>([f64; N]);

impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self {
        Self(data)
    }

    pub fn norm(&self) -> f64 {
        self.0.iter().fold(0.0, |a, e| e * e + a)
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Self {
        // work around limitation in auto_ops with regard to generics
        let m = self.magnitude();
        self.map(|a| a / m)
    }

    pub fn zip<F>(&self, other: impl AsRef<Self>, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let other = other.as_ref();
        let mut out = [0.0; N];
        for i in 0..N {
            out[i] = f(self[i], other[i]);
        }

        Self(out)
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let mut out = [0.0; N];

        for i in 0..N {
            out[i] = f(self[i]);
        }

        Self(out)
    }
}

impl<const N: usize> AsRef<Vector<N>> for Vector<N> {
    fn as_ref(&self) -> &Vector<N> {
        &self
    }
}

impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(a: [f64; N]) -> Self {
        Self(a)
    }
}

impl<const N: usize> TryFrom<Vec<f64>> for Vector<N> {
    type Error = RTError;

    fn try_from(value: Vec<f64>) -> Result<Self> {
        if value.len() != N {
            return Err(RTError::Error(format!(
                "Could not make a Vector<{}> from: {:?}",
                N, value
            )));
        }

        Ok(Self(value.try_into().unwrap()))
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl<const N: usize> Neg for &Vector<N> {
    type Output = Vector<N>;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v: {:?}", self.0)
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Vector2
pub type Vector2 = Vector<2>;

impl Vector2 {
    pub const ZERO: Self = Self([0.0; 2]);
    pub const I: Self = Self([1.0, 0.0]);
    pub const J: Self = Self([0.0, 1.0]);

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn dot(&self, other: impl AsRef<Self>) -> f64 {
        let other = other.as_ref();
        self[0] * other[0] + self[1] * other[1]
    }
}

// So, because auto_ops doesn't support generics yet, we have to have one of
// these blocks per vector type alias
impl_op_ex!(+ |a: &Vector2, b: &Vector2| -> Vector2 { a.zip(b, |x, y| x + y) });
impl_op_ex!(-|a: &Vector2, b: &Vector2| -> Vector2 { a.zip(b, |x, y| x - y) });
impl_op_ex_commutative!(*|a: &Vector2, b: f64| -> Vector2 { a.map(|x| x * b) });
impl_op_ex!(/ |a: &Vector2, b: f64| -> Vector2 { a.map(|x| x / b) });

/// Vector3
pub type Vector3 = Vector<3>;

impl Vector3 {
    pub const ZERO: Self = Self([0.0; 3]);
    pub const I: Self = Self([1.0, 0.0, 0.0]);
    pub const J: Self = Self([0.0, 1.0, 0.0]);
    pub const K: Self = Self([0.0, 0.0, 1.0]);

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn dot(&self, other: impl AsRef<Self>) -> f64 {
        let other = other.as_ref();
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: impl AsRef<Self>) -> Self {
        let Self([bx, by, bz]) = self;
        let Self([cx, cy, cz]) = other.as_ref();

        Self([by * cz - bz * cy, bz * cx - bx * cz, bx * cy - by * cx])
    }
}

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 { a.zip(b, |x, y| x + y) });
// this happens so freqently, that the overhead for the iterator is noticeable
impl_op_ex!(-|a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3::new([
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
    ])
});
impl_op_ex_commutative!(*|a: &Vector3, b: f64| -> Vector3 { a.map(|x| x * b) });
impl_op_ex!(/ |a: &Vector3, b: f64| -> Vector3 { a.map(|x| x / b) });

/// Vector4
pub type Vector4 = Vector<4>;

impl Vector4 {
    pub const ZERO: Self = Self([0.0; 4]);
    pub const I: Self = Self([1.0, 0.0, 0.0, 0.0]);
    pub const J: Self = Self([0.0, 1.0, 0.0, 0.0]);
    pub const K: Self = Self([0.0, 0.0, 1.0, 0.0]);
    pub const W: Self = Self([0.0, 0.0, 0.0, 1.0]);

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn w(&self) -> f64 {
        self[3]
    }

    pub fn dot(&self, other: impl AsRef<Self>) -> f64 {
        let other = other.as_ref();
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }
}

impl_op_ex!(+ |a: &Vector4, b: &Vector4| -> Vector4 { a.zip(b, |x, y| x + y) });
impl_op_ex!(-|a: &Vector4, b: &Vector4| -> Vector4 { a.zip(b, |x, y| x - y) });
impl_op_ex_commutative!(*|a: &Vector4, b: f64| -> Vector4 { a.map(|x| x * b) });
impl_op_ex!(/ |a: &Vector4, b: f64| -> Vector4 { a.map(|x| x / b) });

// Convenience conversions
impl From<Vector2> for Vector3 {
    fn from(v2: Vector2) -> Self {
        Self([v2[0], v2[1], 0.0])
    }
}
impl From<Vector2> for Vector4 {
    fn from(v2: Vector2) -> Self {
        Self([v2[0], v2[1], 0.0, 0.0])
    }
}

impl From<Vector3> for Vector4 {
    fn from(v3: Vector3) -> Self {
        Self([v3[0], v3[1], v3[2], 1.0])
    }
}

impl From<Vector4> for Vector3 {
    fn from(v4: Vector4) -> Self {
        Self([v4[0], v4[1], v4[2]])
    }
}

#[cfg(test)]
mod tests {
    mod vec3 {
        use super::super::*;
        #[test]
        fn negation() {
            let a = Vector3::new([1.0, 10.0, 100.0]);
            let b = Vector3::new([-1.0, -10.0, -100.0]);

            assert_eq!(-a, b);
            assert_eq!(-&a, b);
        }

        #[test]
        fn addition() {
            let a = Vector3::new([1.0, 10.0, 100.0]);
            let b = Vector3::new([2.0, 20.0, 200.0]);
            let c = Vector3::new([3.0, 30.0, 300.0]);

            assert_eq!(a + b, c);
            assert_eq!(a + &b, c);
            assert_eq!(&a + b, c);
            assert_eq!(&a + &b, c);
        }

        #[test]
        fn subtraction() {
            let a = Vector3::new([1.0, 10.0, 100.0]);
            let b = Vector3::new([2.0, 20.0, 200.0]);
            let c = Vector3::new([-1.0, -10.0, -100.0]);

            assert_eq!(a - b, c);
            assert_eq!(a - &b, c);
            assert_eq!(&a - b, c);
            assert_eq!(&a - &b, c);
        }

        #[test]
        fn multiplication() {
            let a = Vector3::new([1.0, 10.0, 100.0]);
            let b = 2.0;
            let c = Vector3::new([2.0, 20.0, 200.0]);

            assert_eq!(a * b, c);
            assert_eq!(&a * b, c);
            assert_eq!(b * a, c);
            assert_eq!(b * &a, c);
        }

        #[test]
        fn division() {
            let a = Vector3::new([1.0, 10.0, 100.0]);
            let b = 2.0;
            let c = Vector3::new([0.5, 5.0, 50.0]);

            assert_eq!(a / b, c);
            assert_eq!(&a / b, c);
        }

        #[test]
        fn dotproduct() {
            let a = Vector3::new([9.0, 2.0, 7.0]);
            let b = Vector3::new([4.0, 8.0, 10.0]);
            let c = 122.0;

            assert_eq!(a.dot(b), c);
            assert_eq!(a.dot(&b), c);
            assert_eq!(b.dot(a), c);
            assert_eq!(b.dot(&a), c);
        }

        #[test]
        fn crossproduct() {
            let a = Vector3::new([2.0, 3.0, 4.0]);
            let b = Vector3::new([5.0, 6.0, 7.0]);
            let c = Vector3::new([-3.0, 6.0, -3.0]);

            assert_eq!(a.cross(b), c);
            assert_eq!(a.cross(&b), c);
        }

        #[test]
        fn magnitude() {
            let a = Vector3::new([2.0, 3.0, 4.0]);
            let b = (a.x() * a.x() + a.y() * a.y() + a.z() * a.z()).sqrt();

            assert_eq!(a.magnitude(), b);
        }

        #[test]
        fn normalize() {
            let a = Vector3::new([2.0, 3.0, 4.0]);
            let b = a / a.magnitude();

            assert_eq!(a.normalize(), b);

            assert_eq!(Vector3::K.normalize(), Vector3::K);
        }
    }
}
