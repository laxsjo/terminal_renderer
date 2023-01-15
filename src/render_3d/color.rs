mod color_ops;

use super::*;
use approx::AbsDiffEq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn from_normal(mut normal: Vec3) -> Self {
        normal = normal.normalize();

        normal /= 2.;
        normal += vec3(0.5, 0.5, 0.5);

        normal.into()
    }

    pub fn normalize(self) -> Self {
        rgb(
            self.r.clamp(0., 1.),
            self.g.clamp(0., 1.),
            self.b.clamp(0., 1.),
        )
    }

    pub fn to_byte_rgb(self) -> ByteRgb {
        self.into()
    }

    /// Convert color to the hsl colorspace.
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::render_3d::*;
    /// use approx::*;
    ///
    /// let color = rgb(0.09, 0.38, 0.46);
    ///
    /// assert_abs_diff_eq!(color.to_hsl(), hsl(0.536, 0.672, 0.275), epsilon = 0.01);
    /// ```
    pub fn to_hsl(self) -> Hsl {
        self.into()
    }
}

impl AbsDiffEq for Rgb {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r.abs_diff_eq(&other.r, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

impl From<Hsl> for Rgb {
    fn from(hsl: Hsl) -> Self {
        let hsl = hsl.normalize();

        // source: https://www.niwa.nu/2013/05/math-behind-colorspace-conversions-rgb-hsl/
        if hsl.s == 0. {
            return rgb(hsl.l, hsl.l, hsl.l);
        }

        let temp_1 = if hsl.l < 0.5 {
            hsl.l * (1. + hsl.s)
        } else {
            hsl.l + hsl.s - hsl.l * hsl.s
        };

        let temp_2 = 2. * hsl.l - temp_1;

        const ONE_THIRD: f32 = 1. / 3.;
        const TWO_THIRDS: f32 = 2. / 3.;

        let temp_r = (hsl.h + ONE_THIRD).rem_euclid(1.);
        let temp_g = hsl.h;
        let temp_b = (hsl.h - ONE_THIRD).rem_euclid(1.);

        fn calc_color_channel(temp_channel: f32, temp_1: f32, temp_2: f32) -> f32 {
            if 6. * temp_channel < 1. {
                temp_2 + (temp_1 - temp_2) * 6. * temp_channel
            } else if 2. * temp_channel < 1. {
                temp_1
            } else if 3. * temp_channel < 2. {
                temp_2 + (temp_1 - temp_2) * (TWO_THIRDS - temp_channel) * 6.
            } else {
                temp_2
            }
        }

        let r = calc_color_channel(temp_r, temp_1, temp_2);
        let g = calc_color_channel(temp_g, temp_1, temp_2);
        let b = calc_color_channel(temp_b, temp_1, temp_2);

        rgb(r, g, b)
    }
}

impl From<Vec3> for Rgb {
    fn from(vec: Vec3) -> Self {
        rgb(vec.x as f32, vec.y as f32, vec.z as f32).normalize()
    }
}

impl From<f32> for Rgb {
    fn from(color: f32) -> Self {
        rgb(color, color, color)
    }
}

impl<T> From<(T, T, T)> for Rgb
where
    T: Into<f32>,
{
    fn from(tuple: (T, T, T)) -> Self {
        rgb(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ByteRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ByteRgb {
    pub fn to_rgb_slice(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    pub fn to_rgba_slice(self, alpha: u8) -> [u8; 4] {
        [self.r, self.g, self.b, alpha]
    }
}

impl From<Rgb> for ByteRgb {
    fn from(rgb: Rgb) -> Self {
        let r = (rgb.r * 255.) as u8;
        let g = (rgb.g * 255.) as u8;
        let b = (rgb.b * 255.) as u8;

        byte_rgb(r, g, b)
    }
}

impl From<Hsl> for ByteRgb {
    fn from(hsl: Hsl) -> Self {
        hsl.into()
    }
}

impl<T> From<(T, T, T)> for ByteRgb
where
    T: Into<u8>,
{
    fn from(tuple: (T, T, T)) -> Self {
        byte_rgb(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

pub fn rgb(r: f32, g: f32, b: f32) -> Rgb {
    Rgb { r, g, b }
}

pub fn byte_rgb(r: u8, g: u8, b: u8) -> ByteRgb {
    ByteRgb { r, g, b }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    pub fn normalize(self) -> Self {
        hsl(
            self.h.rem_euclid(1.),
            self.s.clamp(0., 1.),
            self.l.clamp(0., 1.),
        )
    }

    /// Convert color to the rgb colorspace.
    ///
    /// # Example
    /// ```
    /// use terminal_renderer::render_3d::*;
    /// use approx::*;
    ///
    /// let color = hsl(0.536, 0.672, 0.275);
    ///
    /// assert_abs_diff_eq!(color.to_rgb(), rgb(0.09, 0.38, 0.46), epsilon = 0.01);
    /// ```
    pub fn to_rgb(self) -> Rgb {
        self.into()
    }
}

impl AbsDiffEq for Hsl {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.h.abs_diff_eq(&other.h, epsilon)
            && self.s.abs_diff_eq(&other.s, epsilon)
            && self.l.abs_diff_eq(&other.l, epsilon)
    }
}

impl From<Rgb> for Hsl {
    fn from(rgb: Rgb) -> Self {
        let rgb = rgb.normalize();

        // source: https://www.niwa.nu/2013/05/math-behind-colorspace-conversions-rgb-hsl/
        let min = rgb.r.min(rgb.g.min(rgb.b));
        let max = rgb.r.max(rgb.g.max(rgb.b));

        let l = (min + max) / 2.;

        if min == max {
            return hsl(0., 0., l);
        }

        let s = if l <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2. - max - min)
        };

        let h = if rgb.r >= rgb.g && rgb.r >= rgb.b {
            // red is max
            (rgb.g - rgb.b) / (max - min)
        } else if rgb.g >= rgb.b {
            // green is max
            2. + (rgb.b - rgb.r) / (max - min)
        } else {
            // blue is max
            4. + (rgb.r - rgb.g) / (max - min)
        } / 6.;

        hsl(h, s, l)
    }
}

impl From<Vec3> for Hsl {
    fn from(vec: Vec3) -> Self {
        hsl(vec.x as f32, vec.y as f32, vec.z as f32).normalize()
    }
}

impl<T> From<(T, T, T)> for Hsl
where
    T: Into<f32>,
{
    fn from(tuple: (T, T, T)) -> Self {
        hsl(tuple.0.into(), tuple.1.into(), tuple.2.into())
    }
}

impl Hsl {}

pub fn hsl(h: f32, s: f32, l: f32) -> Hsl {
    Hsl { h, s, l }
}
