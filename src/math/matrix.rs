use std::ops::{Index, IndexMut};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use super::{vector::Vector4, Point3D, Vector3};

/// A 4x4, row-major matrix with associated operations.
///
///
/// For multiplication between matrices and other types ([Point3D](raytracer_rs::math::Point3D) and
/// [Vector3](raytracer_rs::math::Vector3)), points and vectors are treated as column vectors and
/// are premultiplied (other type to the right of the `*` operator). In these cases, points and
/// vectors are temporarily assumed to have a 4th term of 1 to match dimensions.
///
/// Example:
/// ```
/// use raytracer_rs::math::{Matrix4, Point3D, Vector3};
///
/// let m = Matrix4::I;
/// let p = Point3D::from([2.0, 3.0, 4.0]);
/// let v = Vector3::from([20.0, 30.0, 40.0]);
///
/// let mp: Point3D = m * p;
/// let mv: Vector3 = m * v;
///
/// // p * m <- unsupported
/// // v * m <- unsupported
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Matrix4(pub [[f64; 4]; 4]);

impl Matrix4 {
    pub const I: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub fn transpose(&self) -> Self {
        let mut out = Self::default();

        for r in 0..4 {
            for c in 0..4 {
                out[r][c] = self[c][r];
            }
        }

        out
    }

    pub fn det(&self) -> f64 {
        self[0][3] * self[1][2] * self[2][1] * self[3][0]
            - self[0][2] * self[1][3] * self[2][1] * self[3][0]
            - self[0][3] * self[1][1] * self[2][2] * self[3][0]
            + self[0][1] * self[1][3] * self[2][2] * self[3][0]
            + self[0][2] * self[1][1] * self[2][3] * self[3][0]
            - self[0][1] * self[1][2] * self[2][3] * self[3][0]
            - self[0][3] * self[1][2] * self[2][0] * self[3][1]
            + self[0][2] * self[1][3] * self[2][0] * self[3][1]
            + self[0][3] * self[1][0] * self[2][2] * self[3][1]
            - self[0][0] * self[1][3] * self[2][2] * self[3][1]
            - self[0][2] * self[1][0] * self[2][3] * self[3][1]
            + self[0][0] * self[1][2] * self[2][3] * self[3][1]
            + self[0][3] * self[1][1] * self[2][0] * self[3][2]
            - self[0][1] * self[1][3] * self[2][0] * self[3][2]
            - self[0][3] * self[1][0] * self[2][1] * self[3][2]
            + self[0][0] * self[1][3] * self[2][1] * self[3][2]
            + self[0][1] * self[1][0] * self[2][3] * self[3][2]
            - self[0][0] * self[1][1] * self[2][3] * self[3][2]
            - self[0][2] * self[1][1] * self[2][0] * self[3][3]
            + self[0][1] * self[1][2] * self[2][0] * self[3][3]
            + self[0][2] * self[1][0] * self[2][1] * self[3][3]
            - self[0][0] * self[1][2] * self[2][1] * self[3][3]
            - self[0][1] * self[1][0] * self[2][2] * self[3][3]
            + self[0][0] * self[1][1] * self[2][2] * self[3][3]
    }

    pub fn inverse(&self) -> Option<Self> {
        let s0 = self[0][0] * self[1][1] - self[1][0] * self[0][1];
        let s1 = self[0][0] * self[1][2] - self[1][0] * self[0][2];
        let s2 = self[0][0] * self[1][3] - self[1][0] * self[0][3];
        let s3 = self[0][1] * self[1][2] - self[1][1] * self[0][2];
        let s4 = self[0][1] * self[1][3] - self[1][1] * self[0][3];
        let s5 = self[0][2] * self[1][3] - self[1][2] * self[0][3];

        let c5 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let c4 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let c3 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let c2 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let c1 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let c0 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;
        if det == 0.0 {
            return None;
        }

        let mut out = Self::default();

        let invdet = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        out[0][0] = (self[1][1] * c5 - self[1][2] * c4 + self[1][3] * c3) * invdet;
        out[0][1] = (-self[0][1] * c5 + self[0][2] * c4 - self[0][3] * c3) * invdet;
        out[0][2] = (self[3][1] * s5 - self[3][2] * s4 + self[3][3] * s3) * invdet;
        out[0][3] = (-self[2][1] * s5 + self[2][2] * s4 - self[2][3] * s3) * invdet;

        out[1][0] = (-self[1][0] * c5 + self[1][2] * c2 - self[1][3] * c1) * invdet;
        out[1][1] = (self[0][0] * c5 - self[0][2] * c2 + self[0][3] * c1) * invdet;
        out[1][2] = (-self[3][0] * s5 + self[3][2] * s2 - self[3][3] * s1) * invdet;
        out[1][3] = (self[2][0] * s5 - self[2][2] * s2 + self[2][3] * s1) * invdet;

        out[2][0] = (self[1][0] * c4 - self[1][1] * c2 + self[1][3] * c0) * invdet;
        out[2][1] = (-self[0][0] * c4 + self[0][1] * c2 - self[0][3] * c0) * invdet;
        out[2][2] = (self[3][0] * s4 - self[3][1] * s2 + self[3][3] * s0) * invdet;
        out[2][3] = (-self[2][0] * s4 + self[2][1] * s2 - self[2][3] * s0) * invdet;

        out[3][0] = (-self[1][0] * c3 + self[1][1] * c1 - self[1][2] * c0) * invdet;
        out[3][1] = (self[0][0] * c3 - self[0][1] * c1 + self[0][2] * c0) * invdet;
        out[3][2] = (-self[3][0] * s3 + self[3][1] * s1 - self[3][2] * s0) * invdet;
        out[3][3] = (self[2][0] * s3 - self[2][1] * s1 + self[2][2] * s0) * invdet;

        Some(out)
    }

    /// Produces an array of [Vector4] representing the four rows. Yielding [Vector4] simplifies
    /// some of the subsequent operations.
    pub fn rows(&self) -> [Vector4; 4] {
        [
            self[0].into(),
            self[1].into(),
            self[2].into(),
            self[3].into(),
        ]
    }

    /// Produces an array of [Vector4] representing the four columns. Yielding [Vector4] simplifies
    /// some of the subsequent operations.
    pub fn columns(&self) -> [Vector4; 4] {
        [
            [self[0][0], self[1][0], self[2][0], self[3][0]].into(),
            [self[0][1], self[1][1], self[2][1], self[3][1]].into(),
            [self[0][2], self[1][2], self[2][2], self[3][2]].into(),
            [self[0][3], self[1][3], self[2][3], self[3][3]].into(),
        ]
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl_op_ex!(*|a: &Matrix4, b: &Matrix4| -> Matrix4 {
    let mut out = Matrix4::default();

    let rows = a.rows();
    let cols = b.columns();

    for i in 0..4 {
        for j in 0..4 {
            out[i][j] = rows[i].dot(cols[j]);
        }
    }

    out
});

impl_op_ex_commutative!(*|a: &Matrix4, b: f64| -> Matrix4 {
    let mut out = a.clone();
    for i in 0..4 {
        for j in 0..4 {
            out[i][j] *= b;
        }
    }
    out
});

impl_op_ex!(/ |a: &Matrix4, b: f64| -> Matrix4 {
    let mut out = a.clone();
    for i in 0..4 {
        for j in 0..4 {
            out[i][j] /= b;
        }
    }
    out
});

impl_op_ex!(*|a: &Matrix4, b: &Point3D| -> Point3D {
    // we assume the last term in the vec3 -> vec4 would be 1.0
    [
        b.x() * a[0][0] + b.y() * a[0][1] + b.z() * a[0][2] + a[0][3],
        b.x() * a[1][0] + b.y() * a[1][1] + b.z() * a[1][2] + a[1][3],
        b.x() * a[2][0] + b.y() * a[2][1] + b.z() * a[2][2] + a[2][3],
    ]
    .into()
});

impl_op_ex!(*|a: &Matrix4, b: &Vector3| -> Vector3 {
    [
        b.x() * a[0][0] + b.y() * a[0][1] + b.z() * a[0][2],
        b.x() * a[1][0] + b.y() * a[1][1] + b.z() * a[1][2],
        b.x() * a[2][0] + b.y() * a[2][1] + b.z() * a[2][2],
    ]
    .into()
});

#[cfg(test)]
mod tests {
    use crate::math::EPSILON;

    use super::*;

    #[test]
    fn index() {
        let mut a = Matrix4([
            [1.0, -2.0, 3.0, 2.0],
            [2.0, 3.0, 1.0, -1.0],
            [3.0, 3.0, 3.0, 3.0],
            [-1.0, 4.0, 2.0, 1.0],
        ]);

        assert_eq!(a[0], [1.0, -2.0, 3.0, 2.0]);
        assert_eq!(a[1], [2.0, 3.0, 1.0, -1.0]);
        assert_eq!(a[2], [3.0, 3.0, 3.0, 3.0]);
        assert_eq!(a[3], [-1.0, 4.0, 2.0, 1.0]);

        a[0] = [0.0, 0.0, 0.0, 0.0];
        assert_eq!(a[0], [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn determinant() {
        assert_eq!(Matrix4::default().det(), 0.0);
        assert_eq!(Matrix4::I.det(), 1.0);

        let a = Matrix4([
            [1.0, -2.0, 3.0, 2.0],
            [2.0, 3.0, 1.0, -1.0],
            [3.0, 3.0, 3.0, 3.0],
            [-1.0, 4.0, 2.0, 1.0],
        ]);

        assert_eq!(a.det(), -47.0 * 3.0);
    }

    #[test]
    fn inverse() {
        let a = Matrix4([
            [1.0, -2.0, 3.0, 2.0],
            [2.0, 3.0, 1.0, -1.0],
            [3.0, 3.0, 3.0, 3.0],
            [-1.0, 4.0, 2.0, 1.0],
        ]);

        // sanity check
        assert!(a.det() != 0.0);

        let inv = a.inverse();

        assert!(inv.is_some());

        // floating point inaccuracy means we can't actually check this
        // assert_eq!(a * inv.unwrap(), Matrix4::I);
        let r = a * inv.unwrap();
        for i in 0..4 {
            for j in 0..4 {
                assert!((r[i][j] - Matrix4::I[i][j]).abs() < EPSILON);
            }
        }
    }

    #[test]
    fn multiplication() {
        let a = Matrix4([
            [5.0, 7.0, 9.0, 10.0],
            [2.0, 3.0, 3.0, 8.0],
            [8.0, 10.0, 2.0, 3.0],
            [3.0, 3.0, 4.0, 8.0],
        ]);

        let b = Matrix4([
            [3.0, 10.0, 12.0, 18.0],
            [12.0, 1.0, 4.0, 9.0],
            [9.0, 10.0, 12.0, 2.0],
            [3.0, 12.0, 4.0, 10.0],
        ]);

        let c = Matrix4([
            [210.0, 267.0, 236.0, 271.0],
            [93.0, 149.0, 104.0, 149.0],
            [171.0, 146.0, 172.0, 268.0],
            [105.0, 169.0, 128.0, 169.0],
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn scaling() {
        let a = Matrix4([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0, 13.0],
            [14.0, 15.0, 16.0, 17.0],
        ]);

        let b = Matrix4([
            [2.0 * 2.0, 3.0 * 2.0, 4.0 * 2.0, 5.0 * 2.0],
            [6.0 * 2.0, 7.0 * 2.0, 8.0 * 2.0, 9.0 * 2.0],
            [10.0 * 2.0, 11.0 * 2.0, 12.0 * 2.0, 13.0 * 2.0],
            [14.0 * 2.0, 15.0 * 2.0, 16.0 * 2.0, 17.0 * 2.0],
        ]);

        assert_eq!(a * 2.0, b);
        assert_eq!(&a * 2.0, b);
        assert_eq!(2.0 * a, b);
        assert_eq!(2.0 * &a, b);

        assert_eq!(b / 2.0, a);
        assert_eq!(&b / 2.0, a);
    }
}
