use super::*;

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: self.x * -1.,
            y: self.y * -1.,
            z: self.z * -1.,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: self.x * -1.,
            y: self.y * -1.,
        }
    }
}

impl ops::Mul<u32> for UVec2 {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<UVec2> for u32 {
    type Output = UVec2;
    fn mul(self, rhs: UVec2) -> Self::Output {
        UVec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::MulAssign<u32> for UVec2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl ops::Div<u32> for UVec2 {
    type Output = Self;
    fn div(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<u32> for UVec2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl ops::Sub<UVec2> for UVec2 {
    type Output = Self;
    fn sub(self, rhs: UVec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::SubAssign<UVec2> for UVec2 {
    fn sub_assign(&mut self, rhs: UVec2) {
        *self = *self - rhs;
    }
}

impl ops::Add<UVec2> for UVec2 {
    type Output = Self;
    fn add(self, rhs: UVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<UVec2> for UVec2 {
    fn add_assign(&mut self, rhs: UVec2) {
        *self = *self + rhs;
    }
}

impl ops::Mul<i32> for IVec2 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<IVec2> for i32 {
    type Output = IVec2;
    fn mul(self, rhs: IVec2) -> Self::Output {
        IVec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::MulAssign<i32> for IVec2 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl ops::Div<i32> for IVec2 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<i32> for IVec2 {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl ops::Sub<IVec2> for IVec2 {
    type Output = Self;
    fn sub(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::SubAssign<IVec2> for IVec2 {
    fn sub_assign(&mut self, rhs: IVec2) {
        *self = *self - rhs;
    }
}

impl ops::Add<IVec2> for IVec2 {
    type Output = Self;
    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        *self = *self + rhs;
    }
}

impl ops::Neg for IVec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
