use super::*;
use std::ops;

impl ops::Add for Rgb {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl ops::AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Rgb {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign for Rgb {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Rgb {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::MulAssign for Rgb {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Div for Rgb {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl ops::DivAssign for Rgb {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::Rem for Rgb {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r % rhs.r,
            g: self.g % rhs.g,
            b: self.b % rhs.b,
        }
    }
}

impl ops::RemAssign for Rgb {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl ops::Add<f32> for Rgb {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}
impl ops::Add<Rgb> for f32 {
    type Output = Rgb;
    fn add(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self + rhs.r,
            g: self + rhs.g,
            b: self + rhs.b,
        }
    }
}

impl ops::Sub<f32> for Rgb {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        }
    }
}

impl ops::Mul<f32> for Rgb {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl ops::Mul<Rgb> for f32 {
    type Output = Rgb;
    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl ops::Div<f32> for Rgb {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl ops::Rem<f32> for Rgb {
    type Output = Self;
    fn rem(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r % rhs,
            g: self.g % rhs,
            b: self.b % rhs,
        }
    }
}
