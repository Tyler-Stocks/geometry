use std::{error::Error, fmt::Display};

use crate::core::identity::Identity;

pub trait ScalarAdd<'a, 'b>: Sized
where
    Self: 'a + 'b,
{
    fn add(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self;

    fn add_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self;
}

pub trait ScalarMul<'a, 'b>: Sized
where
    Self: 'a + 'b,
{
    fn mul(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self;

    fn mul_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self;
}

pub trait ScalarSub<'a, 'b>: Sized
where
    Self: 'a + 'b,
{
    fn sub(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self;

    fn sub_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self;
}

pub trait ScalarDiv<'a, 'b>: Sized
where
    Self: 'a + 'b,
{
    fn div(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Result<Self, ZeroDivisorError>;

    fn div_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Result<Self, ZeroDivisorError>;
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
        $(impl<'a, 'b> ScalarAdd<'a, 'b> for $implementor {
            fn add(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self {
                lhs.into() + rhs.into()
            }

            fn add_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
                lhs.into() + rhs.into()
            }
        })+
    };
    (sub, $($implementor:ty),+) => {
        $(impl<'a, 'b> ScalarSub<'a, 'b> for $implementor {
            fn sub(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self {
                lhs.into() - rhs.into()
            }

            fn sub_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
                lhs.into() - rhs.into()
            }
        })+
    };
    (mul, $($implementor:ty),+) => {
        $(impl<'a, 'b> ScalarMul<'a, 'b> for $implementor {
            fn mul(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Self {
                lhs.into() * rhs.into()
            }

            fn mul_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Self {
                lhs.into() * rhs.into()
            }
        })+
    };
    (div, $($implementor:ty),+) => {
        $(impl<'a, 'b> ScalarDiv<'a, 'b> for $implementor {
            fn div(lhs: impl Into<&'a Self>, rhs: impl Into<&'b Self>) -> Result<Self, ZeroDivisorError> {
                let lhs = lhs.into();
                let rhs = rhs.into();

                if rhs.is_additive_identity() {
                    return Err(ZeroDivisorError);
                } else {
                    return Ok(lhs / rhs);
                }
            }

            fn div_consuming(lhs: impl Into<Self>, rhs: impl Into<Self>) -> Result<Self, ZeroDivisorError> {
                let lhs = lhs.into();
                let rhs = rhs.into();

                if rhs.is_additive_identity() {
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
