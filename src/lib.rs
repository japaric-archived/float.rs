//! A trait that abstracts over the common functionality of `f32` and `f64`
//!
//! The `Floaty` trait provided in this crate is similar to the deprecated `std::num::Float`, but
//! contains more information about cast operations.

#![deny(warnings)]

#![cfg_attr(feature = "unstable", feature(op_assign_traits))]

extern crate cast;

use std::num::FpCategory;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
#[cfg(feature = "unstable")]
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use cast::From;

macro_rules! self_self {
    ($ty:ty, $($method:ident),+) => {
        $(
            fn $method(self) -> $ty { self.$method() }
         )+
    }
}

macro_rules! self_self_self {
    ($ty:ty, $($method:ident),+) => {
        $(
            fn $method(self, _1: Self) -> $ty { self.$method(_1) }
         )+
    }
}

macro_rules! self_bool {
    ($ty:ty, $($method:ident),+) => {
        $(
            fn $method(self) -> bool { self.$method() }
         )+
    }
}

/// A floating point number
#[cfg(not(feature = "unstable"))]
pub trait Floaty
    : Add<Output=Self>
    + Clone
    + Copy
    + Div<Output=Self>
    + From<f32, Output=Self>
    + From<i16, Output=Self>
    + From<i32, Output=Self>
    + From<i64, Output=Self>
    + From<i8, Output=Self>
    + From<isize, Output=Self>
    + From<u16, Output=Self>
    + From<u32, Output=Self>
    + From<u64, Output=Self>
    + From<u8, Output=Self>
    + From<usize, Output=Self>
    + Mul<Output=Self>
    + Neg<Output=Self>
    + PartialEq
    + PartialOrd
    + Rem<Output=Self>
    + Send
    + Sub<Output=Self>
    + Sync
{
    // `fn(self) -> Self`
    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn trunc(self) -> Self;
    // TODO rust-1.7.0: remove cfg
    #[cfg(feature = "unstable")]
    fn to_degrees(self) -> Self;
    // TODO rust-1.7.0: remove cfg
    #[cfg(feature = "unstable")]
    fn to_radians(self) -> Self;

    // `fn(self, Self) -> Self`
    fn abs_sub(self, Self) -> Self;
    fn atan2(self, Self) -> Self;
    fn hypot(self, Self) -> Self;
    fn log(self, Self) -> Self;
    fn max(self, Self) -> Self;
    fn min(self, Self) -> Self;
    fn powf(self, Self) -> Self;

    // `fn(self) -> bool`
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;

    // Others
    fn classify(self) -> FpCategory;
    fn mul_add(self, Self, Self) -> Self;
    fn powi(self, i32) -> Self;
    fn sin_cos(self) -> (Self, Self);
}

/// A floating point number
#[cfg(feature = "unstable")]
pub trait Floaty
    : Add<Output=Self>
    + AddAssign
    + Clone
    + Copy
    + Div<Output=Self>
    + DivAssign
    + From<f32, Output=Self>
    + From<i16, Output=Self>
    + From<i32, Output=Self>
    + From<i64, Output=Self>
    + From<i8, Output=Self>
    + From<isize, Output=Self>
    + From<u16, Output=Self>
    + From<u32, Output=Self>
    + From<u64, Output=Self>
    + From<u8, Output=Self>
    + From<usize, Output=Self>
    + Mul<Output=Self>
    + MulAssign
    + Neg<Output=Self>
    + PartialEq
    + PartialOrd
    + Rem<Output=Self>
    + RemAssign
    + Send
    + Sub<Output=Self>
    + SubAssign
    + Sync
{
    // `fn(self) -> Self`
    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn trunc(self) -> Self;
    // TODO rust-1.7.0: remove cfg
    #[cfg(feature = "unstable")]
    fn to_degrees(self) -> Self;
    // TODO rust-1.7.0: remove cfg
    #[cfg(feature = "unstable")]
    fn to_radians(self) -> Self;

    // `fn(self, Self) -> Self`
    fn abs_sub(self, Self) -> Self;
    fn atan2(self, Self) -> Self;
    fn hypot(self, Self) -> Self;
    fn log(self, Self) -> Self;
    fn max(self, Self) -> Self;
    fn min(self, Self) -> Self;
    fn powf(self, Self) -> Self;

    // `fn(self) -> bool`
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;

    // Others
    fn classify(self) -> FpCategory;
    fn mul_add(self, Self, Self) -> Self;
    fn powi(self, i32) -> Self;
    fn sin_cos(self) -> (Self, Self);
}

impl Floaty for f32 {
    self_self! {
        f32, abs, acos, acosh, asin, asinh, atan, atanh, cbrt, ceil, cos, cosh, exp, exp2, exp_m1,
        floor, fract, ln, ln_1p, log10, log2, recip, round, signum, sin, sinh, sqrt, tan, tanh,
        trunc
    }

    #[cfg(feature = "unstable")]
    self_self!(f32, to_degrees, to_radians);

    self_self_self! {
        f32, abs_sub, atan2, hypot, log, max, min, powf
    }

    self_bool! {
        f32, is_finite, is_infinite, is_nan, is_normal, is_sign_negative, is_sign_positive
    }

    fn classify(self) -> FpCategory {
        self.classify()
    }
    fn mul_add(self, a: f32, b: f32) -> f32 {
        self.mul_add(a, b)
    }
    fn powi(self, n: i32) -> f32 {
        self.powi(n)
    }
    fn sin_cos(self) -> (f32, f32) {
        self.sin_cos()
    }
}

impl Floaty for f64 {
    self_self! {
        f64, abs, acos, acosh, asin, asinh, atan, atanh, cbrt, ceil, cos, cosh, exp, exp2, exp_m1,
        floor, fract, ln, ln_1p, log10, log2, recip, round, signum, sin, sinh, sqrt, tan, tanh,
        trunc
    }

    #[cfg(feature = "unstable")]
    self_self!(f64, to_degrees, to_radians);

    self_self_self! {
        f64, abs_sub, atan2, hypot, log, max, min, powf
    }

    self_bool! {
        f64, is_finite, is_infinite, is_nan, is_normal, is_sign_negative, is_sign_positive
    }

    fn classify(self) -> FpCategory {
        self.classify()
    }
    fn mul_add(self, a: f64, b: f64) -> f64 {
        self.mul_add(a, b)
    }
    fn powi(self, n: i32) -> f64 {
        self.powi(n)
    }
    fn sin_cos(self) -> (f64, f64) {
        self.sin_cos()
    }
}
