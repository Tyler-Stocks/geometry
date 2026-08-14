pub trait Identity: Sized + Copy {
    fn identity() -> Self;
    fn is_identity(&self) -> bool;
}

macro_rules! impl_identity_int {
    ($($name:ty),*) => {
        $(impl Identity for $name {
            fn identity() -> Self {
                0
            }

            fn is_identity(&self) -> bool {
                *self == Self::identity()
            }
        })*
    };
}

impl_identity_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_identity_float {
    ($($name:ty),*) => {
        $(impl Identity for $name {
            fn identity() -> Self {
                0.0
            }

            fn is_identity(&self) -> bool {
                *self == Self::identity()
            }
        })*
    };
}

impl_identity_float!(f32, f64);
