use std::{error::Error, fmt::Display};

use crate::core::Identity;
use crate::scalar::Scalar;

pub trait ScalarAdd: Sized {
    fn add(lhs: &Self, rhs: &Self) -> Self;

    fn add_consuming(lhs: Self, rhs: Self) -> Self;
}

pub trait ScalarMul: Sized {
    fn mul(lhs: &Self, rhs: &Self) -> Self;

    fn mul_consuming(lhs: Self, rhs: Self) -> Self;
}

pub trait ScalarSub: Sized {
    fn sub(lhs: &Self, rhs: &Self) -> Self;

    fn sub_consuming(lhs: Self, rhs: Self) -> Self;
}

pub trait ScalarDiv: Sized {
    fn div(lhs: &Self, rhs: &Self) -> Result<Self, ZeroDivisorError>;

    fn div_consuming(lhs: Self, rhs: Self) -> Result<Self, ZeroDivisorError>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroDivisorError;

impl Display for ZeroDivisorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot Divide By Zero!")
    }
}

impl Error for ZeroDivisorError {}

macro_rules! impl_scalar_operation {
    (add, $($implementor:ty),+) => {
        $(impl ScalarAdd for $implementor {
            fn add(lhs: &Self, rhs: &Self) -> Self {
                lhs + rhs
            }

            fn add_consuming(lhs: Self, rhs: Self) -> Self {
                lhs + rhs
            }
        })+
    };
    (sub, $($implementor:ty),+) => {
        $(impl ScalarSub for $implementor {
            fn sub(lhs: &Self, rhs: &Self) -> Self {
                lhs - rhs
            }

            fn sub_consuming(lhs: Self, rhs: Self) -> Self {
                lhs - rhs
            }
        })+
    };
    (mul, $($implementor:ty),+) => {
        $(impl ScalarMul for $implementor {
            fn mul(lhs: &Self, rhs: &Self) -> Self {
                lhs * rhs
            }

            fn mul_consuming(lhs: Self, rhs: Self) -> Self {
                lhs * rhs
            }
        })+
    };
    (div, $($implementor:ty),+) => {
        $(impl ScalarDiv for $implementor {
            fn div(lhs: &Self, rhs: &Self) -> Result<Self, ZeroDivisorError> {
                if rhs.is_identity() {
                    return Err(ZeroDivisorError);
                } else {
                    return Ok(lhs / rhs);
                }
            }

            fn div_consuming(lhs: Self, rhs: Self) -> Result<Self, ZeroDivisorError> {
                if rhs.is_identity() {
                    return Err(ZeroDivisorError);
                } else {
                    return Ok(lhs / rhs);
                }
            }
        })+
    };
}

impl_scalar_operation!(
    add, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl_scalar_operation!(
    sub, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl_scalar_operation!(
    mul, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl_scalar_operation!(
    div, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<VALUE> ScalarAdd for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn add(lhs: &Self, rhs: &Self) -> Self {
        Scalar {
            value: VALUE::add(&lhs.value, &rhs.value),
        }
    }

    fn add_consuming(lhs: Self, rhs: Self) -> Self {
        Scalar {
            value: VALUE::add_consuming(lhs.value, rhs.value),
        }
    }
}

impl<VALUE> ScalarSub for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn sub(lhs: &Self, rhs: &Self) -> Self {
        Scalar {
            value: VALUE::sub(&lhs.value, &rhs.value),
        }
    }

    fn sub_consuming(lhs: Self, rhs: Self) -> Self {
        Scalar {
            value: VALUE::sub_consuming(lhs.value, rhs.value),
        }
    }
}

impl<VALUE> ScalarMul for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn mul(lhs: &Self, rhs: &Self) -> Self {
        Scalar {
            value: VALUE::mul(&lhs.value, &rhs.value),
        }
    }

    fn mul_consuming(lhs: Self, rhs: Self) -> Self {
        Scalar {
            value: VALUE::mul_consuming(lhs.value, rhs.value),
        }
    }
}

impl<VALUE> ScalarDiv for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn div(lhs: &Self, rhs: &Self) -> Result<Self, ZeroDivisorError> {
        Ok(Scalar {
            value: VALUE::div(&lhs.value, &rhs.value)?,
        })
    }

    fn div_consuming(lhs: Self, rhs: Self) -> Result<Self, ZeroDivisorError> {
        Ok(Scalar {
            value: VALUE::div_consuming(lhs.value, rhs.value)?,
        })
    }
}
