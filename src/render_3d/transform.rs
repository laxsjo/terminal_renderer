use super::quaternion::*;
use crate::math::*;

pub struct TransformationMatrix {
    pub matrix: Matrix<f32, 4, 4>,
}

impl TransformationMatrix {
    pub fn new(matrix: Matrix<f32, 4, 4>) -> Self {
        Self { matrix }
    }

    pub fn from_3x3_matrix(matrix: Matrix<f32, 3, 3>) -> Self {
        let mut out_matrix = Matrix::new(0.);

        out_matrix[3][3] = 1.;

        out_matrix[0][0] = matrix[0][0];
        out_matrix[0][1] = matrix[0][1];
        out_matrix[0][2] = matrix[0][2];
        out_matrix[1][0] = matrix[1][0];
        out_matrix[1][1] = matrix[1][1];
        out_matrix[1][2] = matrix[1][2];
        out_matrix[2][0] = matrix[2][0];
        out_matrix[2][1] = matrix[2][1];
        out_matrix[2][2] = matrix[2][2];

        Self { matrix: out_matrix }
    }

    // pub fn from_euler_angles(euler: EulerAngles) -> Self {

    // }

    /// Returns the inverse transformation.
    ///
    /// # Panics
    ///
    /// Panics if matrix is invertable (https://en.wikipedia.org/wiki/Invertible_matrix).
    pub fn inverse(&self) -> Self {
        Self::new(self.matrix.inverse().expect("Non invertable matrix given"))
    }

    pub fn combine(&self, other: Self) -> Self {
        Self::new(self.matrix * other.matrix)
    }

    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let transformed = self.matrix * point.to_4matrix();

        transformed.to_vec3()
    }
}

/// Represents an objects location, rotation and scale.
///
/// **Note**: The api design was heavily inspired by Unity's `Transform` class (https://docs.unity3d.com/ScriptReference/Transform.html).
#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion,
}

impl Transform {
    pub const fn identity() -> Self {
        Self {
            position: vec3(0., 0., 0.),
            scale: vec3(1., 1., 1.),
            rotation: Quaternion::identity(),
        }
    }

    pub fn new(position: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn new_position(position: Vec3) -> Self {
        Self {
            position,
            scale: vec3(1., 1., 1.),
            rotation: Quaternion::identity(),
        }
    }

    pub fn new_position_rotation(position: Vec3, rotation: Quaternion) -> Self {
        Self {
            position,
            scale: vec3(1., 1., 1.),
            rotation,
        }
    }

    /// Transform point from local to world space.
    ///
    /// Scale, rotates and then translates point, in that order.
    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        point = point.scale_by(self.scale);

        point = self.rotation.rotate_point(point);

        point += self.position;

        point
    }

    /// Transform point using this transform.
    ///
    /// Inversely translates, rotates and then scales point, in that order.
    pub fn inverse_transform_point(&self, mut point: Vec3) -> Vec3 {
        point -= self.position;

        point = self.rotation.inverse_rotate_point(point);

        point = point.inverse_scale_by(self.scale);

        point
    }

    pub fn translate(&self, coords: Vec3) -> Self {
        Transform {
            position: self.position + coords,
            ..*self
        }
    }

    pub fn translate_mut(&mut self, coords: Vec3) {
        self.position += coords;
    }

    pub fn scale(&self, scale: Vec3) -> Self {
        Transform {
            scale: self.scale.scale_by(scale),
            ..*self
        }
    }

    pub fn scale_mut(&mut self, scale: Vec3) {
        self.scale = self.scale.scale_by(scale);
    }

    pub fn rotate(&self, rotation: Quaternion) -> Self {
        Transform {
            rotation: self.rotation.rotate_by(rotation),
            ..*self
        }
    }

    pub fn rotate_mut(&mut self, rotation: Quaternion) {
        self.rotation.rotate_by_mut(rotation);
    }
}
