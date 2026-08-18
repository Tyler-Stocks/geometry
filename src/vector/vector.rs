use crate::core::identity::Identity;
use crate::core::sqrt::Sqrt;
use crate::core::trig::Cos;
use crate::scalar::ops::*;
use std::error::Error;
use std::rc::Rc;

use std::fmt::Display;
use std::iter::Sum;

pub struct Vector<VALUE, const DIMENSIONS: usize>
where
    for<'a, 'b> VALUE: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity,
{
    pub components: [VALUE; DIMENSIONS],
}

impl<VALUE, const DIMENSIONS: usize> Vector<VALUE, DIMENSIONS>
where
    for<'a, 'b> VALUE: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity,
{
    #[must_use]
    pub fn magnitude(&self) -> f64
    where
        VALUE: Sqrt<Output = f64> + Sum + Into<f64>,
    {
        self.components
            .iter()
            .map(|component| VALUE::mul(component, component))
            .sum::<VALUE>()
            .sqrt()
    }

    #[must_use]
    pub fn angle_between(lhs: &Self, rhs: &Self) -> Result<f64, ZeroMagnitudeError>
    where
        for<'a, 'b> VALUE:
            Sqrt<Output = f64> + Sum + Cos<'a, 'b, Output = f64> + From<f64> + Into<f64>,
    {
        VALUE::acos(
            &VALUE::div_consuming(lhs * rhs, f64::mul(&lhs.magnitude(), &rhs.magnitude()))
                .map_err(|_| ZeroMagnitudeError::new("Angle Between"))?,
        )
        .map_err(|_| ZeroMagnitudeError::new("Angle Between"))
    }
}

impl<SCALAR> Vector<SCALAR, 3>
where
    for<'a, 'b> SCALAR: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity,
{
    #[must_use]
    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        let lhs = lhs.components;
        let rhs = rhs.components;

        Self {
            components: [
                SCALAR::sub(
                    &SCALAR::mul(&lhs[1], &rhs[2]),
                    &SCALAR::mul(&lhs[2], &rhs[1]),
                ),
                SCALAR::sub(
                    &SCALAR::mul(&lhs[2], &rhs[0]),
                    &SCALAR::mul(&lhs[0], &rhs[2]),
                ),
                SCALAR::sub(
                    &SCALAR::mul(&lhs[0], &rhs[1]),
                    &SCALAR::mul(&lhs[1], &rhs[0]),
                ),
            ],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ZeroMagnitudeError {
    pub operation: Rc<str>,
}

impl ZeroMagnitudeError {
    #[must_use]
    pub fn new(operation: impl Into<Rc<str>>) -> Self {
        ZeroMagnitudeError {
            operation: operation.into(),
        }
    }
}

impl Display for ZeroMagnitudeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cannot perform operation {} on a vector with no magnitude!",
            self.operation
        )
    }
}

impl Error for ZeroMagnitudeError {}
