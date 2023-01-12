use super::*;

pub trait Camera {
    /// Get the camera matrix
    /// https://en.wikipedia.org/wiki/Camera_matrix
    fn matrix(&self) -> TransformationMatrix;

    fn position(&self) -> Vec3;
    fn rotation(&self) -> Quaternion;

    fn set_aspect_ratio(&mut self, aspect_ratio: f32);
    fn get_aspect_ratio(&self) -> f32;

    fn aspect_scaling_matrix(&self) -> TransformationMatrix {
        TransformationMatrix::new(Matrix([
            [1., 0., 0., 0.],
            [0., self.get_aspect_ratio(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]))
    }

    fn project_point(&self, mut point: Vec3) -> Vec3 {
        // Logic from here: https://en.wikipedia.org/wiki/Orthographic_projection#Geometry
        let mut matrix = self.matrix();
        let scaling_matrix = self.aspect_scaling_matrix();
        matrix = matrix.combine(scaling_matrix);

        point -= self.position();
        point = self.rotation().inverse_rotate_point(point);

        matrix.transform_point(point)
    }
}

#[derive(Debug)]
pub struct OrthographicCamera {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub width: f32,
    pub height: f32,
    pub far_plane: f32,
    pub near_plane: f32,
    aspect_ratio: f32,
}

impl OrthographicCamera {
    pub fn new(
        transform: Transform,
        width: f32,
        height: f32,
        far_plane: f32,
        near_plane: f32,
    ) -> Self {
        Self {
            position: transform.position,
            rotation: transform.rotation,
            width,
            height,
            far_plane,
            near_plane,
            aspect_ratio: 1.,
        }
    }
}

impl Camera for OrthographicCamera {
    fn position(&self) -> Vec3 {
        self.position
    }
    fn rotation(&self) -> Quaternion {
        self.rotation
    }

    fn matrix(&self) -> TransformationMatrix {
        // Source: https://en.wikipedia.org/wiki/Orthographic_projection#Geometry
        let right = self.width / 2.;
        let left = -right;

        let top = self.height / 2.;
        let bottom = -top;

        // // Why are these negative? Good question ;)
        let far = self.far_plane;
        let near = self.near_plane;

        let mut matrix = Matrix::new(0.);

        matrix[0][0] = 2. / (right - left);
        matrix[1][1] = 2. / (top - bottom);
        matrix[2][2] = -2. / (far - near);

        matrix[0][3] = -((right + left) / (right - left));
        matrix[1][3] = -((top + bottom) / (top - bottom));
        matrix[2][3] = -((far + near) / (far - near));
        matrix[3][3] = 1.;

        TransformationMatrix::new(matrix)
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}

mod tests {
    #[test]
    fn camera_matrix_test_1() {
        use crate::math::*;
        use crate::render_3d::*;
        use approx::*;

        let camera = OrthographicCamera::new(Transform::identity(), 8., 8., 100., 0.01);

        let point = vec3(0.25, 2.64, -1.25);

        assert_abs_diff_eq!(
            camera.project_point(point),
            vec3(0.0625, 0.66, -0.975_197_5)
        );
    }
}
