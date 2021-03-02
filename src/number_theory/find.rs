use crate::number_theory::euclid::ea;

// TODO: generify
#[inline]
pub fn gcd(a: i64, b: i64) -> i64 {
    ea(a, b)
}

// TODO: generify
#[inline]
pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
