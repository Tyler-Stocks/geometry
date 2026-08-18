use assert_eq_float::assert_eq_float;

use super::ops::*;
use super::*;

const EPSILON: f32 = 0.001;

#[test]
fn scalar_add() {
    let lhs = 10.0;
    let rhs = 10.0;

    let add_result = f32::add(&lhs, &rhs);
    assert_eq_float!(add_result, 20.0, EPSILON);

    let add_consuming_result = f32::add_consuming(lhs, rhs);
    assert_eq_float!(add_consuming_result, 20.0, EPSILON);
}

#[test]
fn scalar_sub() {
    let lhs = 10.0;
    let rhs = 10.0;

    let sub_result = f32::sub(&lhs, &rhs);
    assert_eq_float!(sub_result, 0.0, EPSILON);

    let sub_consuming_result = f32::sub_consuming(lhs, rhs);
    assert_eq_float!(sub_consuming_result, 0.0, EPSILON);
}

#[test]
fn scalar_mul() {
    let lhs = 10.0;
    let rhs = 10.0;

    let mul_result = f32::mul(&lhs, &rhs);
    assert_eq_float!(mul_result, 100.0, EPSILON);

    let mul_consuming_result = f32::mul_consuming(lhs, rhs);
    assert_eq_float!(mul_consuming_result, 100.0, EPSILON);
}

#[test]
fn scalar_div() {
    let lhs = 10.0;
    let rhs = 5.00;

    let div_result = f32::div(&lhs, &rhs);
    assert_eq_float!(div_result.unwrap(), 2.0, EPSILON);

    let div_consuming_result = f32::div_consuming(lhs, rhs);
    assert_eq_float!(div_consuming_result.unwrap(), 2.0, EPSILON);
}

#[test]
fn scalar_div_by_zero() {
    let lhs = 1.0;
    let rhs = 0.0;

    assert_eq!(f32::div(&lhs, &rhs), Err(ZeroDivisorError));
    assert_eq!(f32::div_consuming(lhs, rhs), Err(ZeroDivisorError));
}
