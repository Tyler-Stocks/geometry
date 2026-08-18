use std::{
    array,
    iter::Sum,
    ops::{Add, Mul, Sub},
};

use super::vector::Vector;

use crate::core::identity::Identity;
use crate::scalar::ops::*;

impl<VALUE, const DIMENSIONS: usize> Add for Vector<VALUE, DIMENSIONS>
where
    for<'a, 'b> VALUE: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.components.into_iter();
        let mut rhs = rhs.components.into_iter();

        Self {
            components: array::from_fn(|_| {
                VALUE::add_consuming(lhs.next().unwrap(), rhs.next().unwrap())
            }),
        }
    }
}

impl<'a, 'b, VALUE, const DIMENSIONS: usize> Add<&'b Vector<VALUE, DIMENSIONS>>
    for &'a Vector<VALUE, DIMENSIONS>
where
    for<'c, 'd> VALUE: Copy
        + Sized
        + ScalarAdd<'c, 'd>
        + ScalarSub<'c, 'd>
        + ScalarMul<'c, 'd>
        + ScalarDiv<'c, 'd>
        + Identity,
{
    type Output = Vector<VALUE, DIMENSIONS>;

    fn add(self, rhs: &'b Vector<VALUE, DIMENSIONS>) -> Self::Output {
        let mut lhs = self.components.into_iter();
        let mut rhs = rhs.components.into_iter();
        Vector {
            components: array::from_fn(|_| {
                VALUE::add_consuming(lhs.next().unwrap(), rhs.next().unwrap())
            }),
        }
    }
}

impl<VALUE, const DIMENSIONS: usize> Sub for Vector<VALUE, DIMENSIONS>
where
    for<'a, 'b> VALUE: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs = self.components.into_iter();
        let mut rhs = rhs.components.into_iter();

        Self {
            components: array::from_fn(|_| {
                VALUE::add_consuming(lhs.next().unwrap(), rhs.next().unwrap())
            }),
        }
    }
}

impl<'a, 'b, VALUE, const DIMENSIONS: usize> Sub<&'b Vector<VALUE, DIMENSIONS>>
    for &'a Vector<VALUE, DIMENSIONS>
where
    for<'c, 'd> VALUE: Copy
        + Sized
        + ScalarAdd<'c, 'd>
        + ScalarSub<'c, 'd>
        + ScalarMul<'c, 'd>
        + ScalarDiv<'c, 'd>
        + Identity,
{
    type Output = Vector<VALUE, DIMENSIONS>;

    fn sub(self, rhs: &'b Vector<VALUE, DIMENSIONS>) -> Self::Output {
        let mut lhs = self.components.into_iter();
        let mut rhs = rhs.components.into_iter();
        Vector {
            components: array::from_fn(|_| {
                VALUE::add_consuming(lhs.next().unwrap(), rhs.next().unwrap())
            }),
        }
    }
}

impl<VALUE, const DIMENSIONS: usize> Mul for Vector<VALUE, DIMENSIONS>
where
    for<'a, 'b> VALUE: Copy
        + Sized
        + ScalarAdd<'a, 'b>
        + ScalarSub<'a, 'b>
        + ScalarMul<'a, 'b>
        + ScalarDiv<'a, 'b>
        + Identity
        + Sum,
{
    type Output = VALUE;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self.components.iter();
        let rhs = rhs.components.iter();

        lhs.zip(rhs).map(|(lhs, rhs)| VALUE::mul(lhs, rhs)).sum()
    }
}

impl<'a, 'b, VALUE, const DIMENSIONS: usize> Mul<&'b Vector<VALUE, DIMENSIONS>>
    for &'a Vector<VALUE, DIMENSIONS>
where
    for<'c, 'd> VALUE: Copy
        + Sized
        + ScalarAdd<'c, 'd>
        + ScalarSub<'c, 'd>
        + ScalarMul<'c, 'd>
        + ScalarDiv<'c, 'd>
        + Identity
        + Sum,
{
    type Output = VALUE;

    fn mul(self, rhs: &'b Vector<VALUE, DIMENSIONS>) -> Self::Output {
        let lhs = self.components.iter();
        let rhs = rhs.components.iter();

        lhs.zip(rhs).map(|(lhs, rhs)| VALUE::mul(lhs, rhs)).sum()
    }
}
