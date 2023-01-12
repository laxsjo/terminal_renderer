use num::*;
use std::ops;

use super::super::utils::GenericSum;
use super::vec::*;
use super::SignedNum;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix<T: SignedNum, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T: SignedNum, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(cell_value: T) -> Self {
        Matrix([[cell_value; C]; R])
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        // Zero::zero() from here: https://stackoverflow.com/a/73941795/15507414
        let mut out = Matrix::<T, C, R>::new(Zero::zero());

        for i in 0..R {
            for j in 0..C {
                out[j][i] = self[i][j]
            }
        }

        out
    }
}

impl<T: SignedNum, const N: usize> Matrix<T, N, N> {
    pub fn identity() -> Self {
        let mut out = Matrix::new(zero());

        for i in 0..N {
            out[i][i] = one();
        }

        out
    }
}

impl<T: SignedNum, const N: usize> Matrix<T, N, N>
where
    Self: MinorMatrix<T, N, N>,
{
    /// Calculate the cofactor of a matrix.
    ///
    /// The cofactor is calculated by multiplying the minor of the matrix by 1 or -1,
    /// depending on if `i + j` is even or odd, where `i` and `j` are the row and column.
    ///
    /// Source: https://www.cuemath.com/algebra/adjoint-of-a-matrix/
    ///
    /// # Examples
    /// ## Example 1
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [ 3, 6],
    ///     [-4, 8],
    /// ]);
    ///
    /// assert_eq!(a.cofactor(), Matrix([
    ///     [ 8, 4],
    ///     [-6, 3],
    /// ]));
    /// ```
    /// ## Example 2
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [2, -1,  3],
    ///     [0,  5,  2],
    ///     [1, -1, -2],
    /// ]);
    ///
    /// assert_eq!(a.cofactor(), Matrix([
    ///     [ -8,   2, -5],
    ///     [ -5,  -7,  1],
    ///     [-17,  -4, 10],
    /// ]));
    /// ```
    ///
    pub fn cofactor(self) -> Self {
        let mut out = self.minor();

        for i in 0..N {
            for j in 0..N {
                let sign = if (i + j) % 2 == 0 {
                    one::<T>()
                } else {
                    -one::<T>()
                };
                out[i][j] = out[i][j] * sign;
            }
        }

        out
    }

    /// Calculate the Adjoint of a matrix.
    ///
    /// The Adjoint can be found of transposing the cofactor of the matrix.
    ///
    /// Source: https://www.cuemath.com/algebra/adjoint-of-a-matrix/
    pub fn adjoint(self) -> Self {
        self.cofactor().transpose()
    }
}

impl<T: SignedNum> Matrix<T, 2, 2> {
    /// Calculate the determinant of a 2x2 matrix.
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 2],
    ///     [2, 0],
    /// ]);
    ///
    /// assert_eq!(a.determinant(), -4);
    /// ```
    pub fn determinant(self) -> T {
        // From here: https://en.wikipedia.org/wiki/Determinant
        /*
        det |a b|
            |c d|
        = ad - bc
        */
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    /// Calculate the inverse of matrix.
    ///
    /// Returns None if the matrix isn't invertible.
    ///
    /// Read more: https://en.wikipedia.org/wiki/Invertible_matrix
    ///
    // /// # Example
    // /// ```
    // /// use terminal_renderer::math::Matrix;
    // /// use assert_approx_eq::assert_approx_eq;
    // ///
    // /// let a = Matrix([
    // ///     [4., 7.],
    // ///     [2., 6.],
    // /// ]);
    // ///
    // /// assert_eq!(a.inverse(), Some(Matrix([
    // ///     [ 0.6, -0.7],
    // ///     [-0.2,  0.4],
    // /// ])));
    // /// ```
    pub fn inverse(self) -> Option<Self> {
        // Source: https://www.mathsisfun.com/algebra/matrix-inverse.html
        let det = self.determinant();

        if det == zero() {
            return None;
        }

        let inverse =
            Matrix([[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]]) * (one::<T>() / det);

        Some(inverse)
    }
}

impl<T: SignedNum> Matrix<T, 3, 3> {
    /// Calculate the determinant of a 3x3 matrix.
    ///
    /// # Examples
    /// ## Example 1
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 2, 3],
    ///     [3, 2, 1],
    ///     [3, 1, 1],
    /// ]);
    ///
    /// assert_eq!(a.determinant(), -8);
    /// ```
    ///
    /// ## Example 2
    ///
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 1, 0],
    ///     [0, 0, 1],
    ///     [0, 0, 1],
    /// ]);
    ///
    /// assert_eq!(a.determinant(), 0);
    /// ```
    pub fn determinant(self) -> T {
        // From here: https://en.wikipedia.org/wiki/Determinant
        /*
        det |a b c|
            |d e f|
            |g h i|
        = aei + bfg + cdh - ceg - bdi - afh
        */
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][2] * self[1][1] * self[2][0]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][0] * self[1][2] * self[2][1]
    }

    /// Create an matrix identical to self, but with the a row and column removed.
    ///
    /// This is used by this guide to calculate the minor of a matrix:
    /// https://www.cuemath.com/algebra/adjoint-of-a-matrix/
    ///
    /// # Examples
    /// ## Example 1
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(a.block_out_cell(0, 0), Matrix([
    ///     [5, 6],
    ///     [8, 9],
    /// ]));
    /// ```
    ///
    /// ## Example 2
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(a.block_out_cell(1, 1), Matrix([
    ///     [1, 3],
    ///     [7, 9],
    /// ]));
    /// ```
    ///
    /// ## Example 3
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a = Matrix([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(a.block_out_cell(1, 2), Matrix([
    ///     [1, 2],
    ///     [7, 8],
    /// ]));
    /// ```
    ///
    pub fn block_out_cell(self, row: usize, column: usize) -> Matrix<T, 2, 2> {
        let mut out = Matrix::new(zero());

        for i in 0..2 {
            let self_i = i + (i >= row) as usize;
            for j in 0..2 {
                let self_j = j + (j >= column) as usize;

                out[i][j] = self[self_i][self_j];
            }
        }

        out
    }

    /// Calculate the inverse of matrix.
    ///
    /// Returns None if the matrix isn't invertible.
    ///
    /// Read more: https://www.cuemath.com/algebra/inverse-of-3x3-matrix/
    ///
    // /// # Example
    // /// ```
    // /// use terminal_renderer::math::Matrix;
    // /// use assert_approx_eq::assert_approx_eq;
    // ///
    // /// let a = Matrix([
    // ///     [1., 2., 0.],
    // ///     [2., 0., 1.],
    // ///     [0., 0., 1.],
    // /// ]);
    // ///
    // /// assert_eq!(a.inverse().unwrap(), Matrix([
    // ///     [  0.,  0.5, -0.5],
    // ///     [0.5, -0.25, 0.25],
    // ///     [ 0.,    0.,   1.],
    // /// ]));
    // /// ```
    pub fn inverse(self) -> Option<Self> {
        // Source: https://www.cuemath.com/algebra/inverse-of-3x3-matrix/
        let det = self.determinant();

        if det == zero() {
            return None;
        }

        Some(self.adjoint() / det)
    }
}

impl<T: SignedNum> Matrix<T, 4, 4> {
    /// Calculate the determinant of this 4x4 matrix.
    pub fn determinant(&self) -> T {
        // Algorithm monster from here: http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm

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

    /// Calculate the inverse of matrix.
    ///
    /// Returns None if the matrix isn't invertible.
    pub fn inverse(&self) -> Option<Self> {
        // Algorithm monster from here: http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm

        let det = self.determinant();

        if det == zero() {
            return None;
        }

        let mut out = Matrix::new(zero());

        out[0][0] = self[1][2] * self[2][3] * self[3][1] - self[1][3] * self[2][2] * self[3][1]
            + self[1][3] * self[2][1] * self[3][2]
            - self[1][1] * self[2][3] * self[3][2]
            - self[1][2] * self[2][1] * self[3][3]
            + self[1][1] * self[2][2] * self[3][3];
        out[0][1] = self[0][3] * self[2][2] * self[3][1]
            - self[0][2] * self[2][3] * self[3][1]
            - self[0][3] * self[2][1] * self[3][2]
            + self[0][1] * self[2][3] * self[3][2]
            + self[0][2] * self[2][1] * self[3][3]
            - self[0][1] * self[2][2] * self[3][3];
        out[0][2] = self[0][2] * self[1][3] * self[3][1] - self[0][3] * self[1][2] * self[3][1]
            + self[0][3] * self[1][1] * self[3][2]
            - self[0][1] * self[1][3] * self[3][2]
            - self[0][2] * self[1][1] * self[3][3]
            + self[0][1] * self[1][2] * self[3][3];
        out[0][3] = self[0][3] * self[1][2] * self[2][1]
            - self[0][2] * self[1][3] * self[2][1]
            - self[0][3] * self[1][1] * self[2][2]
            + self[0][1] * self[1][3] * self[2][2]
            + self[0][2] * self[1][1] * self[2][3]
            - self[0][1] * self[1][2] * self[2][3];
        out[1][0] = self[1][3] * self[2][2] * self[3][0]
            - self[1][2] * self[2][3] * self[3][0]
            - self[1][3] * self[2][0] * self[3][2]
            + self[1][0] * self[2][3] * self[3][2]
            + self[1][2] * self[2][0] * self[3][3]
            - self[1][0] * self[2][2] * self[3][3];
        out[1][1] = self[0][2] * self[2][3] * self[3][0] - self[0][3] * self[2][2] * self[3][0]
            + self[0][3] * self[2][0] * self[3][2]
            - self[0][0] * self[2][3] * self[3][2]
            - self[0][2] * self[2][0] * self[3][3]
            + self[0][0] * self[2][2] * self[3][3];
        out[1][2] = self[0][3] * self[1][2] * self[3][0]
            - self[0][2] * self[1][3] * self[3][0]
            - self[0][3] * self[1][0] * self[3][2]
            + self[0][0] * self[1][3] * self[3][2]
            + self[0][2] * self[1][0] * self[3][3]
            - self[0][0] * self[1][2] * self[3][3];
        out[1][3] = self[0][2] * self[1][3] * self[2][0] - self[0][3] * self[1][2] * self[2][0]
            + self[0][3] * self[1][0] * self[2][2]
            - self[0][0] * self[1][3] * self[2][2]
            - self[0][2] * self[1][0] * self[2][3]
            + self[0][0] * self[1][2] * self[2][3];
        out[2][0] = self[1][1] * self[2][3] * self[3][0] - self[1][3] * self[2][1] * self[3][0]
            + self[1][3] * self[2][0] * self[3][1]
            - self[1][0] * self[2][3] * self[3][1]
            - self[1][1] * self[2][0] * self[3][3]
            + self[1][0] * self[2][1] * self[3][3];
        out[2][1] = self[0][3] * self[2][1] * self[3][0]
            - self[0][1] * self[2][3] * self[3][0]
            - self[0][3] * self[2][0] * self[3][1]
            + self[0][0] * self[2][3] * self[3][1]
            + self[0][1] * self[2][0] * self[3][3]
            - self[0][0] * self[2][1] * self[3][3];
        out[2][2] = self[0][1] * self[1][3] * self[3][0] - self[0][3] * self[1][1] * self[3][0]
            + self[0][3] * self[1][0] * self[3][1]
            - self[0][0] * self[1][3] * self[3][1]
            - self[0][1] * self[1][0] * self[3][3]
            + self[0][0] * self[1][1] * self[3][3];
        out[2][3] = self[0][3] * self[1][1] * self[2][0]
            - self[0][1] * self[1][3] * self[2][0]
            - self[0][3] * self[1][0] * self[2][1]
            + self[0][0] * self[1][3] * self[2][1]
            + self[0][1] * self[1][0] * self[2][3]
            - self[0][0] * self[1][1] * self[2][3];
        out[3][0] = self[1][2] * self[2][1] * self[3][0]
            - self[1][1] * self[2][2] * self[3][0]
            - self[1][2] * self[2][0] * self[3][1]
            + self[1][0] * self[2][2] * self[3][1]
            + self[1][1] * self[2][0] * self[3][2]
            - self[1][0] * self[2][1] * self[3][2];
        out[3][1] = self[0][1] * self[2][2] * self[3][0] - self[0][2] * self[2][1] * self[3][0]
            + self[0][2] * self[2][0] * self[3][1]
            - self[0][0] * self[2][2] * self[3][1]
            - self[0][1] * self[2][0] * self[3][2]
            + self[0][0] * self[2][1] * self[3][2];
        out[3][2] = self[0][2] * self[1][1] * self[3][0]
            - self[0][1] * self[1][2] * self[3][0]
            - self[0][2] * self[1][0] * self[3][1]
            + self[0][0] * self[1][2] * self[3][1]
            + self[0][1] * self[1][0] * self[3][2]
            - self[0][0] * self[1][1] * self[3][2];
        out[3][3] = self[0][1] * self[1][2] * self[2][0] - self[0][2] * self[1][1] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][0] * self[1][2] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            + self[0][0] * self[1][1] * self[2][2];

        out = out * (one::<T>() / det);

        Some(out)
    }
}

impl Matrix<f32, 2, 1> {
    pub fn to_vec2(self) -> Vec2 {
        Vec2 {
            x: self[0][0],
            y: self[1][0],
        }
    }
}
impl Matrix<f32, 3, 1> {
    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self[0][0],
            y: self[1][0],
            z: self[2][0],
        }
    }
}

impl Matrix<f32, 4, 1> {
    /// Convert to vec3 using first three rows, ignoring the fourth.
    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self[0][0],
            y: self[1][0],
            z: self[2][0],
        }
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::Index<usize> for Matrix<T, R, C> {
    type Output = [T; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::Add<&Self> for Matrix<T, R, C> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] + rhs[i][j];
            }
        }

        self
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::Sub<&Self> for Matrix<T, R, C> {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] - rhs[i][j];
            }
        }

        self
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::Mul<T> for Matrix<T, R, C> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] * rhs;
            }
        }

        self
    }
}

impl<T: SignedNum, const R: usize, const C: usize> ops::Div<T> for Matrix<T, R, C> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..R {
            for j in 0..C {
                self[i][j] = self[i][j] / rhs;
            }
        }

        self
    }
}

/* This is very confusing, use this if you need help: https://en.wikipedia.org/wiki/Matrix_multiplication
# Diagram
self:    rhs:     output:
     N        P        P
M[...] * N[...] = M[...]

# Example
M = 5, N = 3, P = 4
               N                                             P
 ⎡ ... ... ... ⎤                      P    ⎡ ... ... ... ... ⎤
 ⎥ ... ... ... ⎢    ⎡ ... ... ... ... ⎤    ⎥ ... ... ... ... ⎢
 ⎥ ... ... ... ⎢ *  ⎥ ... ... ... ... ⎢ =  ⎥ ... ... ... ... ⎢
 ⎥ ... ... ... ⎢   N⎣ ... ... ... ... ⎦    ⎥ ... ... ... ... ⎢
M⎣ ... ... ... ⎦                          M⎣ ... ... ... ... ⎦
*/
impl<T: SignedNum, const M: usize, const N: usize, const P: usize> ops::Mul<Matrix<T, N, P>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T, M, P>;
    /// # Examples
    ///
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a: Matrix<i32, 2, 3> = Matrix([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ]);
    /// let b: Matrix<i32, 3, 2> = Matrix([
    ///     [7,  8 ],
    ///     [9,  10],
    ///     [11, 12],
    /// ]);
    ///
    /// assert_eq!(
    ///     a * b,
    ///     Matrix([
    ///         [58,  64 ],
    ///         [139, 154],
    ///     ])
    /// );
    /// ```
    ///
    /// ```
    /// use terminal_renderer::math::Matrix;
    ///
    /// let a: Matrix<i32, 4, 2> = Matrix([
    ///     [1, 2],
    ///     [3, 4],
    ///     [5, 6],
    ///     [7, 8],
    /// ]);
    /// let b: Matrix<i32, 2, 3> = Matrix([
    ///     [9,  10, 11],
    ///     [12, 13, 14],
    /// ]);
    ///
    /// assert_eq!(
    ///     a * b,
    ///     Matrix([
    ///         [33,  36,  39],
    ///         [75,  82,  89],
    ///         [117, 128, 139],
    ///         [159, 174, 189],
    ///     ])
    /// );
    /// ```
    ///
    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut out = Matrix::new(zero());

        for i in 0..M {
            for j in 0..P {
                let sum = (0..N).map(|k| self[i][k] * rhs[k][j]).generic_sum();
                out[i][j] = sum;
            }
        }

        out
    }
}

impl ops::Mul<Vec3> for Matrix<f32, 3, 3> {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (self * rhs.to_matrix()).to_vec3()
    }
}

impl ops::Mul<Vec2> for Matrix<f32, 2, 2> {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        (self * rhs.to_matrix()).to_vec2()
    }
}

pub trait MinorMatrix<T: SignedNum, const R: usize, const C: usize> {
    fn minor(self) -> Self;
}

impl<T: SignedNum> MinorMatrix<T, 2, 2> for Matrix<T, 2, 2> {
    /// Calculate the minor of this matrix.
    ///
    /// Explanation of what the minor of a matrix is can be found here:
    /// https://www.cuemath.com/algebra/adjoint-of-a-matrix/
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::math::Matrix;
    /// use terminal_renderer::math::MinorMatrix;
    ///
    /// let a = Matrix([
    ///     [3, 6],
    ///     [-4, 8],
    /// ]);
    ///
    /// assert_eq!(a.minor(), Matrix([
    ///     [8, -4],
    ///     [6, 3],
    /// ]))
    /// ```
    fn minor(self) -> Self {
        // Source: https://www.cuemath.com/algebra/adjoint-of-a-matrix/
        Matrix([[self[1][1], self[1][0]], [self[0][1], self[0][0]]])
    }
}

impl<T: SignedNum> MinorMatrix<T, 3, 3> for Matrix<T, 3, 3> {
    /// Calculate the minor of this matrix.
    /// Explanation of what the minor of a matrix is can be found here:
    /// https://www.cuemath.com/algebra/adjoint-of-a-matrix/
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::math::Matrix;
    /// use terminal_renderer::math::MinorMatrix;
    ///
    /// let a = Matrix([
    ///     [2, -1,  3],
    ///     [0,  5,  2],
    ///     [1, -1, -2],
    /// ]);
    ///
    /// assert_eq!(a.minor(), Matrix([
    ///     [-8, -2, -5],
    ///     [ 5, -7, -1],
    ///     [-17, 4, 10],
    /// ]))
    /// ```
    fn minor(self) -> Self {
        let mut out = Matrix::new(zero());

        for i in 0..3 {
            for j in 0..3 {
                let determinant = self.block_out_cell(i, j).determinant();
                out[i][j] = determinant;
            }
        }

        out
    }
}
