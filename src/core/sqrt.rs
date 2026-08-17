pub trait Sqrt {
    #[must_use]
    fn sqrt(&self) -> f64;
}

macro_rules! impl_sqrt {
    ($($name:ty),+) => {
        $(impl Sqrt for $name {
            fn sqrt(&self) -> f64 {
                f64::sqrt((*self).into())
            }
        })+
    };
}

impl_sqrt!(f32, f64, u8, u16, u32, i8, i16, i32);
