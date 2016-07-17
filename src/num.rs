//! Numeric helpers
use std::ops::Mul;
use super::structure::Number;
use std::f32::consts::PI;

pub fn square<T: Mul+Copy>(x: T) -> T::Output {
    x * x
}

pub fn angle_dist(o: Number) -> Number {
    let r = o % 2. * PI;
    if r > PI { 2. * PI - r } else { r }
}

/// A max() function that only requires a partial ordering.
///
/// Necessary for floats because they don't implement a total ordering, something that the regular
/// Iterator::max() function needs.
pub fn partial_max<I: Iterator>(iter: I) -> Option<I::Item>
    where I::Item : PartialOrd {
    iter.fold(None, |ret, x| {
        match ret {
            None => Some(x),
            Some(ref y) if x > *y => Some(x),
            _ => ret
        }
    })
}

