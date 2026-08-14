use std::iter::Sum;

use super::ops::*;

use crate::core::identity::Identity;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Scalar<T>
where
    T: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    pub value: T,
}

impl<VALUE> Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    #[must_use]
    pub fn add(lhs: &Self, rhs: &Self) -> Self {
        Self {
            value: VALUE::add(&lhs.value, &rhs.value),
        }
    }

    #[must_use]
    pub fn sub(lhs: &Self, rhs: &Self) -> Self {
        Self {
            value: VALUE::sub(&lhs.value, &rhs.value),
        }
    }

    #[must_use]
    pub fn mul(lhs: &Self, rhs: &Self) -> Self {
        Self {
            value: VALUE::mul(&lhs.value, &rhs.value),
        }
    }

    #[must_use]
    pub fn div(lhs: &Self, rhs: &Self) -> Result<Self, ZeroDivisorError> {
        Ok(Self {
            value: VALUE::div(&lhs.value, &rhs.value)?,
        })
    }

    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.value.is_identity()
    }
}

impl<VALUE> From<VALUE> for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn from(value: VALUE) -> Self {
        Self { value }
    }
}

macro_rules! impl_from_scalar {
    ($($name:ty),+) => {
        $(
            impl From<Scalar<$name>> for $name {
                fn from(value: Scalar<$name>) -> $name {
                    value.value
                }
            }
        )+
    };
}

impl_from_scalar!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64
);

impl<VALUE> Sum for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Scalar {
                value: VALUE::identity(),
            },
            |acc, value| Scalar::add(&acc, &value),
        )
    }
}

impl<'a, VALUE> Sum<&'a Scalar<VALUE>> for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity,
{
    fn sum<I: Iterator<Item = &'a Scalar<VALUE>>>(iter: I) -> Self {
        iter.fold(
            Scalar {
                value: VALUE::identity(),
            },
            |acc, value| Scalar::add(&acc, value),
        )
    }
}
