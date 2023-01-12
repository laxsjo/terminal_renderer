use num;
use std::ops;

mod general;
pub mod matrix;
pub mod vec;

pub use general::*;
pub use matrix::*;
pub use vec::*;

pub use num_traits::Float;

/// Generic trait for any floating point number or integer.
pub trait Num: num::Num + num::NumCast + Copy + PartialOrd {}
impl<T: num::Num + num::NumCast + Copy + PartialOrd> Num for T {}

/// Generic trait for any floating point number or signed integer.
pub trait SignedNum: num::Num + num::NumCast + ops::Neg<Output = Self> + Copy + PartialOrd {}
impl<T: num::Num + num::NumCast + ops::Neg<Output = Self> + Copy + PartialOrd> SignedNum for T {}

// pub fn to_radians<T: Float>(degrees: T) -> T {
//     // Source: https://www.rapidtables.com/convert/number/degrees-to-radians.html
//     // & https://keisan.casio.com/calculator
//     degrees * num_traits::float::Float::
// }

// pub fn to_degrees<T: Float>(radians: T) -> T {
//     radians * 57.2957795130823208768
// }
