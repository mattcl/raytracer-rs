use crate::math::{Matrix4, Vector3};

/// Transform is a builder for generating tranformation matrices. Invoking
/// `.build()` will yield a [Matrix4](crate::math::Matrix4)
/// Example:
/// ```
/// use raytracer_rs::math::Transform;
/// let matrix = Transform::new()
///     .rotate_z(20.0)
///     .translate([0.0, 2.0, 0.0].into())
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    rotations: Vec<Matrix4>,
    scale: Matrix4,
    trans: Vector3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotations: Vec::new(),
            scale: Matrix4::I,
            trans: Vector3::default(),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rotate `degrees` about the X-axis. Rotations are applied in the order
    /// they are specified. Multiple rotations on a given axis are permitted.
    /// Example:
    /// ```
    /// use raytracer_rs::math::Transform;
    /// let x_then_y = Transform::new()
    ///     .rotate_x(20.0) // will rotate x first
    ///     .rotate_y(44.0) // then y
    ///     .build();
    ///
    /// let y_then_x = Transform::new()
    ///     .rotate_y(44.0) // will rotate y first
    ///     .rotate_x(20.0) // then x
    ///     .build();
    /// ```
    pub fn rotate_x<T>(&mut self, degrees: T) -> &mut Self
    where
        T: Into<f64>,
    {
        let th = degrees.into().to_radians();
        self.rotations.push(Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, th.cos(), -th.sin(), 0.0],
            [0.0, th.sin(), th.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
        self
    }

    /// Rotate `degrees` about the Y-axis. Rotations are applied in the order
    /// they are specified. Multiple rotations on a given axis are permitted.
    /// Example:
    /// ```
    /// use raytracer_rs::math::Transform;
    /// let x_then_y = Transform::new()
    ///     .rotate_x(20.0) // will rotate x first
    ///     .rotate_y(44.0) // then y
    ///     .build();
    ///
    /// let y_then_x = Transform::new()
    ///     .rotate_y(44.0) // will rotate y first
    ///     .rotate_x(20.0) // then x
    ///     .build();
    /// ```
    pub fn rotate_y<T>(&mut self, degrees: T) -> &mut Self
    where
        T: Into<f64>,
    {
        let th = degrees.into().to_radians();
        self.rotations.push(Matrix4([
            [th.cos(), 0.0, th.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-th.sin(), 0.0, th.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
        self
    }

    /// Rotate `degrees` about the Z-axis. Rotations are applied in the order
    /// they are specified. Multiple rotations on a given axis are permitted.
    /// Example:
    /// ```
    /// use raytracer_rs::math::Transform;
    /// let z_then_y = Transform::new()
    ///     .rotate_z(20.0) // will rotate z first
    ///     .rotate_y(44.0) // then y
    ///     .build();
    ///
    /// let z_then_x = Transform::new()
    ///     .rotate_z(44.0) // will rotate y first
    ///     .rotate_x(20.0) // then z
    ///     .build();
    /// ```
    pub fn rotate_z<T>(&mut self, degrees: T) -> &mut Self
    where
        T: Into<f64>,
    {
        let th = degrees.into().to_radians();
        self.rotations.push(Matrix4([
            [th.cos(), -th.sin(), 0.0, 0.0],
            [th.sin(), th.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));
        self
    }

    /// Uniformly scale by `val` (will override all previously set scale values). Scale happens
    /// pre-rotation.
    pub fn scale<T>(&mut self, val: T) -> &mut Self
    where
        T: Into<f64>,
    {
        let val = val.into();
        self.scale[0][0] = val;
        self.scale[1][1] = val;
        self.scale[2][2] = val;
        self
    }

    /// Scale `val` units in the X direction. Will leave scale values in other
    /// directions unchanged. Scale happens pre-rotation.
    pub fn scale_x<T>(&mut self, val: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.scale[0][0] = val.into();
        self
    }

    /// Scale `val` units in the Y direction. Will leave scale values in other
    /// directions unchanged. Scale happens pre-rotation.
    pub fn scale_y<T>(&mut self, val: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.scale[1][1] = val.into();
        self
    }

    /// Scale `val` units in the Z direction. Will leave scale values in other
    /// directions unchanged. Scale happens pre-rotation.
    pub fn scale_z<T>(&mut self, val: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.scale[2][2] = val.into();
        self
    }

    pub fn translate(&mut self, vec: Vector3) -> &mut Self {
        self.trans = vec;
        self
    }

    /// Produce the final transformation matrix from this builder. The builder is not consumed.
    pub fn build(&self) -> Matrix4 {
        let mut r = self.rotations.iter().rev().fold(Matrix4::I, |a, e| a * e) * self.scale;
        r[0][3] = self.trans[0];
        r[1][3] = self.trans[1];
        r[2][3] = self.trans[2];

        r
    }
}
