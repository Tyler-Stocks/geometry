use std::ops::Div;

pub trait Identity: Sized + Copy {
    #[must_use]
    fn additive_identity() -> Self;

    #[must_use]
    fn is_additive_identity(&self) -> bool;

    #[must_use]
    fn multiplicative_identity() -> Self;

    #[must_use]
    fn is_multiplicative_identity(&self) -> bool;
}

macro_rules! impl_identity_int {
    ($($name:ty),*) => {
        $(impl Identity for $name {
            fn additive_identity() -> Self {
                0
            }

            fn is_additive_identity(&self) -> bool {
                *self == <$name>::additive_identity()
            }

            fn multiplicative_identity() -> Self {
                1
            }

            fn is_multiplicative_identity(&self) -> bool {
                *self == <$name>::multiplicative_identity()
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
            fn additive_identity() -> Self {
                0.0
            }

            fn is_additive_identity(&self) -> bool {
                *self == <$name>::additive_identity()
            }

            fn multiplicative_identity() -> Self {
                1.0
            }

            fn is_multiplicative_identity(&self) -> bool {
                *self == <$name>::multiplicative_identity()
            }
        })*
    };
}

impl_identity_float!(f32, f64);
