use assert_eq_float::assert_eq_float;

use super::ops::*;
use super::*;

const EPSILON: f32 = 0.001;

#[test]
fn scalar_add() {
    let lhs: Scalar<f32> = 10.0.into();
    let rhs: Scalar<f32> = 10.0.into();

    let add_result = Scalar::add(&lhs, &rhs);
    assert_eq_float!(add_result.value, 20.0, EPSILON);

    let add_consuming_result = Scalar::add_consuming(lhs, rhs);
    assert_eq_float!(add_consuming_result.value, 20.0, EPSILON);
}

#[test]
fn scalar_sub() {
    let lhs: Scalar<f32> = 10.0.into();
    let rhs: Scalar<f32> = 10.0.into();

    let sub_result = Scalar::sub(&lhs, &rhs);
    assert_eq_float!(sub_result.value, 0.0, EPSILON);

    let sub_consuming_result = Scalar::sub_consuming(lhs, rhs);
    assert_eq_float!(sub_consuming_result.value, 0.0, EPSILON);
}

#[test]
fn scalar_mul() {
    let lhs: Scalar<f32> = 10.0.into();
    let rhs: Scalar<f32> = 10.0.into();

    let mul_result = Scalar::mul(&lhs, &rhs);
    assert_eq_float!(mul_result.value, 100.0, EPSILON);

    let mul_consuming_result = Scalar::mul_consuming(lhs, rhs);
    assert_eq_float!(mul_consuming_result.value, 100.0, EPSILON);
}

#[test]
fn scalar_div() {
    let lhs: Scalar<f32> = 10.0.into();
    let rhs: Scalar<f32> = 5.00.into();

    let div_result = Scalar::div(&lhs, &rhs);
    assert_eq_float!(div_result.unwrap().value, 2.0, EPSILON);

    let div_consuming_result = Scalar::div_consuming(lhs, rhs);
    assert_eq_float!(div_consuming_result.unwrap().value, 2.0, EPSILON);
}

#[test]
fn scalar_div_by_zero() {
    let lhs: Scalar<f32> = 1.0.into();
    let rhs: Scalar<f32> = 0.0.into();

    assert_eq!(Scalar::div(&lhs, &rhs), Err(ZeroDivisorError));
    assert_eq!(Scalar::div_consuming(lhs, rhs), Err(ZeroDivisorError));
}
