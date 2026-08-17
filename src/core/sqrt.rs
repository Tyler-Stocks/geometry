use crate::{
    core::identity::Identity,
    scalar::{Scalar, ops::*},
};

pub trait Sqrt {
    type Output;

    #[must_use]
    fn sqrt(&self) -> Self::Output;
}

macro_rules! impl_sqrt {
    ($($name:ty),+) => {
        $(impl Sqrt for $name {
            type Output = f64;

            fn sqrt(&self) -> Self::Output {
                f64::sqrt((*self).into())
            }
        })+
    };
}

impl_sqrt!(f32, f64, u8, u16, u32, i8, i16, i32);

impl<VALUE> Sqrt for Scalar<VALUE>
where
    VALUE: Copy + Sized + ScalarAdd + ScalarSub + ScalarMul + ScalarDiv + Identity + Into<f64>,
{
    type Output = Scalar<f64>;

    fn sqrt(&self) -> Self::Output {
        Scalar {
            value: f64::sqrt(self.value.into()),
        }
    }
}
