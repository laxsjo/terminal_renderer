mod vec_ops;

use num::integer::Roots;
use num_traits::AsPrimitive;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};
// use vec_ops::*;
// use approx

use super::*;
use std::{
    fmt::{Debug, Display, Write},
    ops,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const X_AXIS: Vec3 = vec3(1., 0., 0.);
    pub const Y_AXIS: Vec3 = vec3(0., 1., 0.);
    pub const Z_AXIS: Vec3 = vec3(0., 0., 1.);
    pub const ZERO: Vec3 = vec3(0., 0., 0.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Checks if any component is NaN.
    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn to_matrix(self) -> Matrix<f32, 3, 1> {
        Matrix([[self.x], [self.y], [self.z]])
    }
    pub fn to_4matrix(self) -> Matrix<f32, 4, 1> {
        Matrix([[self.x], [self.y], [self.z], [1.]])
    }

    /// Returns the magnitude on self,
    /// or in other words the length of the vector.
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes this vector.
    /// This makes it's magnitude (or length) be equal to 1.
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::math::*;
    /// use approx::*;
    ///
    /// let vec = vec3(-1.508, -3.230, -1.815);
    ///
    /// assert_abs_diff_eq!(vec.normalize(), vec3(-0.377, -0.807, -0.454), epsilon = 0.001);
    /// ```
    pub fn normalize(self) -> Vec3 {
        self / self.magnitude()
    }

    pub fn dot_product(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross product with self and other vector.
    ///
    /// # Examples
    /// example 1
    /// ```
    /// use terminal_renderer::math::*;
    /// use approx::*;
    ///
    /// let vec_a = vec3(2., 3., 4.);
    /// let vec_b = vec3(5., 6., 7.);
    ///
    /// assert_abs_diff_eq!(vec_a.cross_product(vec_b), vec3(-3., 6., -3.), epsilon = 0.001);
    /// ```
    ///
    /// example 2
    ///
    /// ```
    /// use terminal_renderer::math::*;
    /// use approx::*;
    ///
    /// let vec_a = vec3(0.381, -1.094, 1.630);
    /// let vec_b = vec3(-1.432, -0.652, 2.350);
    ///
    /// assert_abs_diff_eq!(vec_a.cross_product(vec_b), vec3(-1.508, -3.230, -1.815), epsilon = 0.001);
    /// ```
    pub fn cross_product(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Scale this vector, along the x, y, and z axis using another vector.
    ///
    /// This is achieved by multiplying the two vectors component wise.
    pub fn scale_by(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    /// Scale this vector, along the x, y, and z axis using the inverse of another vector.
    ///
    /// This is achieved by dividing self's components by other's component wise.
    pub fn inverse_scale_by(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }
    pub fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }
}

impl<T> From<(T, T, T)> for Vec3
where
    T: Into<f32>,
{
    fn from(tuple: (T, T, T)) -> Self {
        vec3(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

impl approx::AbsDiffEq for Vec3 {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs(self.x - other.x) <= epsilon
            && f32::abs(self.y - other.y) <= epsilon
            && f32::abs(self.z - other.z) <= epsilon
    }
}

impl PartialEq for Vec3 {
    // Haha! You can't stop me with your 'best practices'! :P
    fn eq(&self, other: &Self) -> bool {
        self.x.total_cmp(&other.x).is_eq()
            && self.y.total_cmp(&other.y).is_eq()
            && self.z.total_cmp(&other.z).is_eq()
    }
}

impl Eq for Vec3 {}

// impl PartialOrd for Vec3 {

// }

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .total_cmp(&other.x)
            .then(self.y.total_cmp(&other.y))
            .then(self.z.total_cmp(&other.z))
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;

        std::fmt::Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        std::fmt::Display::fmt(&self.y, f)?;
        f.write_str(", ")?;
        std::fmt::Display::fmt(&self.z, f)?;

        f.write_char(')')
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Checks if any component is NaN.
    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn to_matrix(self) -> Matrix<f32, 2, 1> {
        Matrix([[self.x], [self.y]])
    }

    /// Returns the magnitude on self,
    /// or in other words the length of the vector.
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalizes this vector.
    /// This makes it's magnitude (or length) be equal to 1.
    pub fn normalize(self) -> Vec2 {
        self / self.magnitude()
    }

    pub fn dot_product(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Scale this vector, along the x and y axis using another vector.
    ///
    /// This is achieved by multiplying the two vectors component wise.
    pub fn scale_vec(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn swap(self) -> Self {
        vec2(self.y, self.x)
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }
}

impl From<UVec2> for Vec2 {
    fn from(a: UVec2) -> Self {
        Self {
            x: a.x as f32,
            y: a.y as f32,
        }
    }
}

impl From<IVec2> for Vec2 {
    fn from(a: IVec2) -> Self {
        Self {
            x: a.x as f32,
            y: a.y as f32,
        }
    }
}

impl<T> From<(T, T)> for Vec2
where
    T: Into<f32>,
{
    fn from(tuple: (T, T)) -> Self {
        vec2(tuple.0.into(), tuple.1.into())
    }
}

impl<T> From<PhysicalSize<T>> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn from(size: PhysicalSize<T>) -> Self {
        vec2(size.width.as_(), size.height.as_())
    }
}

impl<T> From<PhysicalPosition<T>> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn from(size: PhysicalPosition<T>) -> Self {
        vec2(size.x.as_(), size.y.as_())
    }
}

impl<T> From<LogicalSize<T>> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn from(size: LogicalSize<T>) -> Self {
        vec2(size.width.as_(), size.height.as_())
    }
}

impl<T> From<LogicalPosition<T>> for Vec2
where
    T: AsPrimitive<f32>,
{
    fn from(size: LogicalPosition<T>) -> Self {
        vec2(size.x.as_(), size.y.as_())
    }
}

impl approx::AbsDiffEq for Vec2 {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs(self.x - other.x) <= epsilon && f32::abs(self.y - other.y) <= epsilon
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;

        std::fmt::Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        std::fmt::Display::fmt(&self.y, f)?;

        f.write_char(')')
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}
impl UVec2 {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Returns the magnitude on self,
    /// or in other words the length of the vector.
    pub fn magnitude(self) -> u32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalizes this vector.
    /// This makes it's magnitude (or length) be equal to 1.
    pub fn normalize(self) -> UVec2 {
        self / self.magnitude()
    }

    pub fn dot_product(self, other: Self) -> u32 {
        self.x * other.x + self.y * other.y
    }

    /// Scale this vector, along the x and y axis using another vector.
    ///
    /// This is achieved by multiplying the two vectors component wise.
    pub fn scale_vec(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn swap(self) -> Self {
        uvec2(self.y, self.x)
    }
}

impl From<Vec2> for UVec2 {
    fn from(a: Vec2) -> Self {
        Self {
            x: a.x as u32,
            y: a.y as u32,
        }
    }
}

impl<T> From<(T, T)> for UVec2
where
    T: Into<u32>,
{
    fn from(tuple: (T, T)) -> Self {
        uvec2(tuple.0.into(), tuple.1.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the magnitude on self,
    /// or in other words the length of the vector.
    pub fn magnitude(self) -> i32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalizes this vector.
    /// This makes it's magnitude (or length) be equal to 1.
    pub fn normalize(self) -> IVec2 {
        self / self.magnitude()
    }

    pub fn dot_product(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Scale this vector, along the x and y axis using another vector.
    ///
    /// This is achieved by multiplying the two vectors component wise.
    pub fn scale_vec(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn swap(self) -> Self {
        ivec2(self.y, self.x)
    }
}

impl From<Vec2> for IVec2 {
    fn from(a: Vec2) -> Self {
        Self {
            x: a.x as i32,
            y: a.y as i32,
        }
    }
}

impl From<UVec2> for IVec2 {
    fn from(a: UVec2) -> Self {
        Self {
            x: a.x as i32,
            y: a.y as i32,
        }
    }
}

impl<T> From<(T, T)> for IVec2
where
    T: Into<i32>,
{
    fn from(tuple: (T, T)) -> Self {
        ivec2(tuple.0.into(), tuple.1.into())
    }
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}
pub const fn uvec2(x: u32, y: u32) -> UVec2 {
    UVec2::new(x, y)
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}
