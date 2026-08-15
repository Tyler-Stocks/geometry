use std::{error::Error, fmt::Display};

use super::identity::Identity;
use crate::{core::inverse::Inverse, scalar::ops::*};

pub trait Sin {
    #[must_use]
    fn sin(&self) -> f64;

    #[must_use]
    fn asin(&self) -> Result<f64, OutOfBounds>;

    #[must_use]
    fn sinh(&self) -> f64;

    #[must_use]
    fn asinh(&self) -> Result<f64, OutOfBounds>;
}

macro_rules! impl_sin {
    ($($name:ty),+) => {
        $(
            impl Sin for $name {

                fn sin(&self) -> f64 {
                    f64::sin((*self).into())
                }

                fn asin(&self) -> Result<f64, OutOfBounds> {
                    let result = f64::asin((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }

                fn sinh(&self) -> f64 {
                    f64::sinh((*self).into())
                }

                fn asinh(&self) -> Result<f64, OutOfBounds> {
                    let result = f64::asinh((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }
            }
        )+
    };
}

impl_sin!(f32, f64, u8, u16, u32, i8, i16, i32);

pub trait Cos {
    #[must_use]
    fn cos(&self) -> f64;

    #[must_use]
    fn acos(&self) -> Result<f64, OutOfBounds>;

    #[must_use]
    fn cosh(&self) -> f64;

    #[must_use]
    fn acosh(&self) -> Result<f64, OutOfBounds>;
}

macro_rules! impl_cos {
    ($($name:ty),+) => {
        $(
            impl Cos for $name {
                fn cos(&self) -> f64 {
                    f64::cos((*self).into())
                }

                fn acos(&self) -> Result<f64, OutOfBounds> {
                    let result = f64::acos((*self).into());
                    return if result.is_nan() { Err(OutOfBounds) } else { Ok(result) }
                }

                fn cosh(&self) -> f64 {
                    f64::cosh((*self).into())
                }

                fn acosh(&self) -> Result<f64, OutOfBounds> {
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

pub trait Csc: Sin + Inverse {
    #[must_use]
    fn csc(&self) -> f64;

    #[must_use]
    fn acsc(&self) -> Result<f64, OutOfBounds>;

    #[must_use]
    fn csch(&self) -> f64;

    #[must_use]
    fn acsch(&self) -> Result<f64, OutOfBounds>;
}

macro_rules! impl_csc {
    ($($name:ty),+) => {
        $(impl Csc for $name {
            fn csc(&self) -> f64 {
                self.sin().inverse()
            }

            fn acsc(&self) -> Result<f64, OutOfBounds> {
                Ok(self.asin()?.inverse())
            }

            fn csch(&self) -> f64 {
                self.sinh().inverse()
            }

            fn acsch(&self) -> Result<f64, OutOfBounds> {
                Ok(self.asinh()?.inverse())
            }
        })+
    };
}

impl_csc!(f32, f64, u8, u16, u32, i8, i16, i32);

pub trait Sec: Cos + Inverse {
    #[must_use]
    fn sec(&self) -> f64;

    #[must_use]
    fn asec(&self) -> Result<f64, OutOfBounds>;

    #[must_use]
    fn sech(&self) -> f64;

    #[must_use]
    fn asech(&self) -> Result<f64, OutOfBounds>;
}

macro_rules! impl_sec {
    ($($name:ty),+) => {
        $(impl Sec for $name {
            fn sec(&self) -> f64 {
                self.cos().inverse()
            }

            fn asec(&self) -> Result<f64, OutOfBounds> {
                Ok(self.acos()?.inverse())
            }

            fn sech(&self) -> f64 {
                self.cosh().inverse()
            }

            fn asech(&self) -> Result<f64, OutOfBounds> {
                Ok(self.acosh()?.inverse())
            }
        })+
    };
}

impl_sec!(f32, f64, u8, u16, u32, i8, i16, i32);

pub trait Cot: Tan + Inverse {
    #[must_use]
    fn cot(&self) -> f64;

    #[must_use]
    fn acot(&self) -> Result<f64, OutOfBounds>;

    #[must_use]
    fn coth(&self) -> f64;

    #[must_use]
    fn acoth(&self) -> Result<f64, OutOfBounds>;
}

macro_rules! impl_cot {
    ($($name:ty),+) => {
        $(impl Cot for $name {
            fn cot(&self) -> f64 {
                self.tan().inverse()
            }

            fn acot(&self) -> Result<f64, OutOfBounds> {
                Ok(self.atan()?.inverse())
            }

            fn coth(&self) -> f64 {
                self.tanh().inverse()
            }

            fn acoth(&self) -> Result<f64, OutOfBounds> {
                Ok(self.atanh()?.inverse())
            }
        })+
    };
}

impl_cot!(f32, f64, u8, u16, u32, i8, i16, i32);

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
