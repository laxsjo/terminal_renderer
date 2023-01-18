use crate::math::*;
use approx::AbsDiffEq;
use std::f32::consts;
use std::ops;

pub struct EulerAngles {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Represents a rotation in 3D-space.
///
/// Uses a quarternion representation internally.
///
/// Algorithms from here: https://danceswithcode.net/engineeringnotes/quaternions/quaternions.html
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self {
        w: 1.,
        x: 0.,
        y: 0.,
        z: 0.,
    };

    /// Returns the identity rotation: (1, 0, 0, 0).
    pub const fn identity() -> Self {
        Quaternion {
            w: 1.,
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /// Create a rotation of angle around axis.
    ///
    /// angle is in radians
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half = angle / 2.;
        Self {
            w: half.cos(),
            x: axis.x * half.sin(),
            y: axis.y * half.sin(),
            z: axis.z * half.sin(),
        }
    }

    /// Calculates the axis angle representation.
    ///
    /// The returned angle is in radians.
    ///
    /// Returns the axis Vec3(1, 0, 0) if angle is zero.
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let angle = 2. * self.w.acos();

        if angle == 0. || angle.is_nan() {
            return (Vec3::new(1., 0., 0.), angle);
        }

        let half_angle_sin = (angle / 2.).sin();

        let axis = Vec3::new(
            self.x / half_angle_sin,
            self.y / half_angle_sin,
            self.z / half_angle_sin,
        );

        (axis, angle)
    }

    pub fn from_rotation_matrix(matrix: Matrix<f32, 3, 3>) -> Self {
        let q0_abs = ((1. + matrix[0][0] + matrix[1][1] + matrix[2][2]) / 4.).sqrt();
        let q1_abs = ((1. + matrix[0][0] - matrix[1][1] - matrix[2][2]) / 4.).sqrt();
        let q2_abs = ((1. - matrix[0][0] + matrix[1][1] - matrix[2][2]) / 4.).sqrt();
        let q3_abs = ((1. - matrix[0][0] - matrix[1][1] + matrix[2][2]) / 4.).sqrt();

        let (q0, q1, q2, q3);

        if q0_abs >= q1_abs && q0_abs >= q2_abs && q0_abs >= q3_abs {
            q0 = q0_abs;
            q1 = (matrix[2][1] - matrix[1][2]) / (4. * q0);
            q2 = (matrix[0][2] - matrix[2][0]) / (4. * q0);
            q3 = (matrix[1][0] - matrix[0][1]) / (4. * q0);
        } else if q1_abs >= q0_abs && q1_abs >= q2_abs && q1_abs >= q3_abs {
            q1 = q1_abs;
            q0 = (matrix[2][1] - matrix[1][2]) / (4. * q1);
            q2 = (matrix[0][1] - matrix[1][0]) / (4. * q1);
            q3 = (matrix[0][2] - matrix[2][0]) / (4. * q1);
        } else if q2_abs >= q0_abs && q2_abs >= q1_abs && q2_abs >= q3_abs {
            q2 = q2_abs;
            q0 = (matrix[0][2] - matrix[2][0]) / (4. * q2);
            q1 = (matrix[0][1] - matrix[1][0]) / (4. * q2);
            q3 = (matrix[1][2] - matrix[2][1]) / (4. * q2);
        } else {
            q3 = q3_abs;
            q0 = (matrix[1][0] - matrix[0][1]) / (4. * q3);
            q1 = (matrix[0][2] - matrix[2][0]) / (4. * q3);
            q2 = (matrix[1][2] - matrix[2][1]) / (4. * q3);
        }

        Self {
            w: q0,
            x: q1,
            y: q2,
            z: q3,
        }
    }

    /// Calculates the rotation matrix representation of this rotation.
    pub fn to_rotation_matrix(&self) -> Matrix<f32, 3, 3> {
        // Source: https://danceswithcode.net/engineeringnotes/quaternions/quaternions.html
        let Quaternion {
            w: q0,
            x: q1,
            y: q2,
            z: q3,
        } = self;

        let q0q1 = 2. * q0 * q1;
        let q0q2 = 2. * q0 * q2;
        let q0q3 = 2. * q0 * q3;

        let q1q2 = 2. * q1 * q2;
        let q1q3 = 2. * q1 * q3;

        let q2q3 = 2. * q2 * q3;

        Matrix([
            [
                1. - 2. * q2.powi(2) - 2. * q3.powi(2),
                q1q2 - q0q3,
                q1q3 + q0q2,
            ],
            [
                q1q2 + q0q3,
                1. - 2. * q1.powi(2) - 2. * q3.powi(2),
                q2q3 - q0q1,
            ],
            [
                q1q3 - q0q2,
                q2q3 + q0q1,
                1. - 2. * q1.powi(2) - 2. * q2.powi(2),
            ],
        ])
    }

    /// Creates a rotation from euler angles.
    ///
    /// The angles `z`, `y`, and `x` are rotations around the corresponding axes,
    /// applied in that order.
    ///
    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        let sin_x = (x / 2.).sin();
        let cos_x = (x / 2.).cos();

        let sin_y = (y / 2.).sin();
        let cos_y = (y / 2.).cos();

        let sin_z = (z / 2.).sin();
        let cos_z = (z / 2.).cos();

        Self {
            w: cos_x * cos_y * cos_z + sin_x * sin_y * sin_z,
            x: sin_x * cos_y * cos_z - cos_x * sin_y * sin_z,
            y: cos_x * sin_y * cos_z + sin_x * cos_y * sin_z,
            z: cos_x * cos_y * sin_z - sin_x * sin_y * cos_z,
        }
    }

    /// Calculate the euler angle representation of this rotation.
    ///
    /// Euler angles consists of three rotations around the z, y, and x axis
    /// in that order.
    pub fn to_euler_angles(&self) -> EulerAngles {
        let y = (2. * (self.w * self.y - self.x * self.z)).asin();
        // Test for gimbal lock
        // **Note**: these comparisons should probably check if approx equal,
        // since y should only at most equal pi/2?
        // Investigate if you have problems
        if y >= consts::FRAC_PI_2 {
            return EulerAngles {
                x: 0.,
                y,
                z: -2. * self.x.atan2(self.w),
            };
        } else if y <= -consts::FRAC_PI_2 {
            return EulerAngles {
                x: 0.,
                y,
                z: 2. * self.x.atan2(self.w),
            };
        }

        let x = (2. * (self.w * self.x + self.y * self.z))
            .atan2(self.w.powi(2) - self.x.powi(2) - self.y.powi(2) + self.z.powi(2));

        let z = (2. * (self.w * self.z + self.x * self.y))
            .atan2(self.w.powi(2) + self.x.powi(2) - self.y.powi(2) - self.z.powi(2));

        EulerAngles { x, y, z }
    }

    pub fn from_look_rotation(forward: Vec3, up: Vec3) -> Self {
        // Source: https://gamefaqs.gamespot.com/boards/210-game-design-and-programming/71121728 (the last comment)
        // and the associated pastebin: https://pastebin.com/ubATCxJY
        // Primary source: https://answers.unity.com/questions/467614/what-is-the-source-code-of-quaternionlookrotation.html

        let forward = forward.normalize();
        let right = up.cross_product(forward).normalize();
        let up = forward.cross_product(right);

        // dbg!(&forward);
        // dbg!(&right);
        // dbg!(&up);

        let mut matrix = Matrix::new(0.);

        matrix[0][0] = right.x;
        matrix[0][1] = right.y;
        matrix[0][2] = right.z;

        matrix[1][0] = up.x;
        matrix[1][1] = up.y;
        matrix[1][2] = up.z;

        matrix[2][0] = forward.x;
        matrix[2][1] = forward.y;
        matrix[2][2] = forward.z;

        Self::from_rotation_matrix(matrix)
    }

    /// Calculates the inverse of this rotation.
    ///
    /// The inverse of a rotation performs the opposite rotation.
    pub fn inverse(mut self) -> Self {
        self.x *= -1.;
        self.y *= -1.;
        self.z *= -1.;

        self
    }

    /// Calculates the inverse of this rotation in place, and then returns a
    /// mutable reference to self.
    ///
    /// The inverse of a rotation performs the opposite rotation.
    pub fn inverse_mut(&mut self) -> &mut Self {
        self.x *= -1.;
        self.y *= -1.;
        self.z *= -1.;

        self
    }

    /// Rotates self by another rotation.
    pub fn rotate_by(self, other: Self) -> Self {
        other * self
    }

    /// Rotates self by another rotation in place, and then returns mutable
    /// reference to self.  
    pub fn rotate_by_mut(&mut self, other: Self) -> &mut Self {
        *self = other * *self;

        self
    }

    /// Rotates self by another rotation defined by an axis and angle.
    pub fn rotate_axis_angle(self, axis: Vec3, angle: f32) -> Self {
        let other = Self::from_axis_angle(axis, angle);

        self.rotate_by(other)
    }

    /// Rotates self by another rotation defined by an axis and angle, and then
    /// returns mutable reference to self.
    pub fn rotate_axis_angle_mut(&mut self, axis: Vec3, angle: f32) -> &mut Self {
        let other = Self::from_axis_angle(axis, angle);

        self.rotate_by_mut(other)
    }

    /// Rotate point using this rotation.
    ///
    /// This function uses *active* rotation, where the point is rotated with
    /// respect to the coordinate system.
    ///
    /// For *passive* rotation, see `inverse_transform_point`.
    pub fn rotate_point(&self, point: Vec3) -> Vec3 {
        let p = Quaternion {
            w: 0.,
            x: point.x,
            y: point.y,
            z: point.z,
        };

        let rotated = self.inverse() * p * *self;

        Vec3 {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    }

    /// Inversely rotate point using this rotation.
    /// This function is the inverse of `transform_point`, it applies the
    /// rotation but in reverse.
    ///
    /// It uses *passive* rotation, where the coordinate system is rotated with
    /// respect to the point.
    ///
    /// For *active* rotation, see `transform_point`.
    pub fn inverse_rotate_point(&self, point: Vec3) -> Vec3 {
        let p = Quaternion {
            w: 0.,
            x: point.x,
            y: point.y,
            z: point.z,
        };

        let rotated = *self * p * self.inverse();

        Vec3 {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.rotate_point(Vec3::Z_AXIS)
    }
    pub fn up(&self) -> Vec3 {
        self.rotate_point(Vec3::Y_AXIS)
    }
    pub fn right(&self) -> Vec3 {
        self.rotate_point(Vec3::X_AXIS)
    }
}

impl ops::Mul for Quaternion {
    type Output = Self;
    /// Calculate the product of to rotation quarternions.
    ///
    /// This effectively combines the two rotations into one.
    fn mul(self, rhs: Self) -> Self::Output {
        let r0 = self.w;
        let r1 = self.x;
        let r2 = self.y;
        let r3 = self.z;

        let s0 = rhs.w;
        let s1 = rhs.x;
        let s2 = rhs.y;
        let s3 = rhs.z;

        Quaternion {
            w: r0 * s0 - r1 * s1 - r2 * s2 - r3 * s3,
            x: r0 * s1 + r1 * s0 - r2 * s3 + r3 * s2,
            y: r0 * s2 + r1 * s3 + r2 * s0 - r3 * s1,
            z: r0 * s3 - r1 * s2 + r2 * s1 + r3 * s0,
        }
    }
}

impl AbsDiffEq for Quaternion {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.w.abs_diff_eq(&other.w, epsilon)
            && self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

mod tests {
    #[test]
    fn test_euler_angles_1() {
        use super::Quaternion;
        use crate::math::*;
        use approx;

        let rotation = Quaternion::from_euler_angles(0., 0., 25.0_f32.to_radians());

        let point = vec3(1., -1., 1.);

        approx::assert_abs_diff_eq!(
            rotation.rotate_point(point),
            vec3(1.32893, -0.483689, 1.),
            epsilon = 0.0001
        )
    }

    #[test]
    fn test_euler_angles_2() {
        use super::Quaternion;
        use crate::math::*;
        use approx;

        let rotation =
            Quaternion::from_euler_angles(25.0.to_radians(), 45.0.to_radians(), 25.0.to_radians());

        let point = vec3(1., -1., 1.);

        approx::assert_abs_diff_eq!(
            rotation.rotate_point(point),
            vec3(1.51246, -0.761036, -0.365087),
            epsilon = 0.0001
        )
    }

    #[test]
    fn test_axis_angle_1() {
        use super::Quaternion;
        use crate::math::*;
        use approx;

        let rotation = Quaternion::from_axis_angle(Vec3::Z_AXIS, 90.0.to_radians());

        let point = vec3(1., -1., 1.);

        approx::assert_abs_diff_eq!(
            rotation.rotate_point(point),
            vec3(1., 1., 1.),
            epsilon = 0.0001
        )
    }

    // #[test]
    // fn test_from_matrix_1() {
    //     use super::Quaternion;
    //     use crate::math::*;
    //     use approx::*;

    //     let matrix = Matrix([[0., 0., -1.], [0., 1., 0.], [1., 0., 0.]]);

    //     assert_abs_diff_eq!(
    //         Quaternion::from_rotation_matrix(matrix),
    //         Quaternion {
    //             w: 0.7071068286895752,
    //             x: 0.,
    //             y: -0.7071068286895752,
    //             z: 0.
    //         },
    //         epsilon = 0.0001
    //     );
    // }

    #[test]
    fn test_from_look_rotation_1() {
        use super::Quaternion;
        use crate::math::*;
        use approx::*;
        use std::f32::consts;

        let forward = Vec3::X_AXIS;

        let rotation = Quaternion::from_look_rotation(forward, Vec3::Y_AXIS);

        assert_abs_diff_eq!(
            rotation,
            Quaternion {
                w: consts::FRAC_1_SQRT_2,
                x: 0.,
                y: -consts::FRAC_1_SQRT_2,
                z: 0.
            },
            epsilon = 0.0001
        );

        // let matrix1 = Matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        // let matrix2 = Matrix([[0, 0, -1], [0, 1, 0], [1, 0, 0]])
    }
}
