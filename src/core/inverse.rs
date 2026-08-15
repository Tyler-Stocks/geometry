use core::ops::Div;

use super::identity::Identity;

pub trait Inverse: Div + Sized + Identity + Copy {
    fn inverse(&self) -> Self;
}

macro_rules! impl_inverse {
    ($($name:ty),+) => {
       $(impl Inverse for $name {
           fn inverse(&self) -> Self {
               <$name>::multiplicative_identity() / *self
           }
       })+
    };
}

impl_inverse!(
    f32, f64, usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
);
