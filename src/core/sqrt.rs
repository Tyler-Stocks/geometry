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
