use std::{error::Error, fmt::Display};

use super::identity::Identity;
use crate::scalar::ops::*;

pub trait Sin: Into<f64> {
    type Output: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity;

    #[must_use]
    fn sin(&self) -> Self::Output;

    #[must_use]
    fn asin(&self) -> Result<Self::Output, OutOfBounds>;

    #[must_use]
    fn sinh(&self) -> Self::Output;

    #[must_use]
    fn asinh(&self) -> Result<Self::Output, OutOfBounds>;
}

macro_rules! impl_sin {
    ($($name:ty),+) => {
        $(
            impl Sin for $name {
                type Output = f64;

                fn sin(&self) -> Self::Output {
                    f64::sin((*self).into())
                }

                fn asin(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::asin((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }

                fn sinh(&self) -> Self::Output {
                    f64::sinh((*self).into())
                }

                fn asinh(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::asinh((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }
            }
        )+
    };
}

impl_sin!(f32, f64, u8, u16, u32, i8, i16, i32);

pub trait Cos {
    type Output: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity;

    #[must_use]
    fn cos(&self) -> Self::Output;

    #[must_use]
    fn acos(&self) -> Result<Self::Output, OutOfBounds>;

    #[must_use]
    fn cosh(&self) -> Self::Output;

    #[must_use]
    fn acosh(&self) -> Result<Self::Output, OutOfBounds>;
}

macro_rules! impl_cos {
    ($($name:ty),+) => {
        $(
            impl Cos for $name {
                type Output = f64;

                fn cos(&self) -> Self::Output {
                    f64::cos((*self).into())
                }

                fn acos(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::acos((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }

                fn cosh(&self) -> Self::Output {
                    f64::cosh((*self).into())
                }

                fn acosh(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::acosh((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }
            }
        )+
    };
}

impl_cos!(f32, f64, u8, u16, u32, i8, i16, i32);

pub trait Tan {
    type Output: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity;

    #[must_use]
    fn tan(&self) -> Self::Output;

    #[must_use]
    fn atan(&self) -> Result<Self::Output, OutOfBounds>;

    #[must_use]
    fn tanh(&self) -> Self::Output;

    #[must_use]
    fn atanh(&self) -> Result<Self::Output, OutOfBounds>;
}

macro_rules! impl_tan {
    ($($name:ty),+) => {
        $(
            impl Tan for $name {
                type Output = f64;

                fn tan(&self) -> Self::Output {
                    f64::tan((*self).into())
                }

                fn atan(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::atan((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }

                fn tanh(&self) -> Self::Output {
                    f64::tanh((*self).into())
                }

                fn atanh(&self) -> Result<Self::Output, OutOfBounds> {
                    let result = f64::atanh((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }
            }
        )+
    };
}

impl_tan!(f32, f64, u8, u16, u32, i8, i16, i32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct OutOfBounds;

impl Display for OutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input To Inverse Trig Functions Must Be In The Range -1.0, 1.0!"
        )
    }
}

impl Error for OutOfBounds {}
