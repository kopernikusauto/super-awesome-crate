#![warn(clippy::all)]

use num_traits::{Float, FloatConst};

/// Normalize an angle from an arbitrary range to [-π; π]
///
/// # Examples
/// ```
/// use std::f64::consts::PI;
/// use super_awesome_crate::normalize_angle;
/// normalize_angle(-3.1 * PI);
/// ```
pub fn normalize_angle<T: Float + FloatConst + std::ops::SubAssign + std::ops::AddAssign + std::ops::Neg>(angle: T) -> T {
    let mut normalized_angle: T = angle;
    let pi: T = FloatConst::PI();
    normalized_angle =
        normalized_angle.signum() * (normalized_angle.abs() % FloatConst::TAU());
    if normalized_angle >= pi {
        normalized_angle -= FloatConst::TAU();
    } else if normalized_angle < -pi {
        normalized_angle += FloatConst::TAU();
    }
    normalized_angle
}

/// Returns the denormalized angle `new` based on the value of the `previous` angle
///
/// # Arguments
///
/// * `previous` - the previous angle in an arbitrary range [-∞, ∞]
/// * `new` - the newly received angle in range [-π; π] which directly follows `previous`
///
/// # Examples
/// ```
/// use std::f64::consts::PI;
/// use super_awesome_crate::denormalize_angle;
/// denormalize_angle(-0.9 * PI, -0.7 * PI);
/// ```
pub fn denormalize_angle<T: Float + FloatConst + std::ops::SubAssign + std::ops::AddAssign>(previous: T, new: T) -> T {
    let mut denormalized_angle = new;
    while denormalized_angle > previous + FloatConst::PI() {
        denormalized_angle -= FloatConst::TAU();
    }

    while denormalized_angle < previous - FloatConst::PI() {
        denormalized_angle += FloatConst::TAU();
    }

    denormalized_angle
}

#[test]
fn angle_normalization() {
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    assert_relative_eq!(normalize_angle(-3.1 * PI), 0.9 * PI);
    assert_relative_eq!(normalize_angle(-2.1 * PI), -0.1 * PI);
    assert_relative_eq!(normalize_angle(-1.1 * PI), 0.9 * PI);
    assert_relative_eq!(normalize_angle(-0.1 * PI), -0.1 * PI);

    assert_relative_eq!(normalize_angle(0.1 * PI), 0.1 * PI);
    assert_relative_eq!(normalize_angle(1.1 * PI), -0.9 * PI);
    assert_relative_eq!(normalize_angle(2.1 * PI), 0.1 * PI);
    assert_relative_eq!(normalize_angle(3.1 * PI), -0.9 * PI);
}

#[test]
fn angle_alignment() {
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    assert_relative_eq!(denormalize_angle(-0.9 * PI, -0.7 * PI), -0.7 * PI);
    assert_relative_eq!(denormalize_angle(-0.9 * PI, 0.7 * PI), -1.3 * PI);
    assert_relative_eq!(denormalize_angle(-0.9 * PI, -1.3 * PI), -1.3 * PI);

    assert_relative_eq!(denormalize_angle(0.9 * PI, -0.7 * PI), 1.3 * PI);
    assert_relative_eq!(denormalize_angle(0.9 * PI, 0.7 * PI), 0.7 * PI);
    assert_relative_eq!(denormalize_angle(0.9 * PI, 1.3 * PI), 1.3 * PI);

    assert_relative_eq!(denormalize_angle(1.1 * PI, -0.9 * PI), 1.1 * PI);
    assert_relative_eq!(denormalize_angle(-1.1 * PI, 0.9 * PI), -1.1 * PI);
    assert_relative_eq!(denormalize_angle(2.1 * PI, -0.1 * PI), 1.9 * PI);
    assert_relative_eq!(denormalize_angle(3.1 * PI, -0.9 * PI), 3.1 * PI);
    assert_relative_eq!(denormalize_angle(-3.1 * PI, 0.9 * PI), -3.1 * PI);
}