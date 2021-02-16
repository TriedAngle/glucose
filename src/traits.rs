use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// I don't know if this is the `best` solution yet, but I do not like to write everything 10x
// so this should do for now
// this does not implement `Copy` because maybe some other types are `Clone` only.
pub trait MathComponent<T>:
    Default
    + Add<Output = T>
    + AddAssign
    + Sub<Output = T>
    + SubAssign
    + Mul<Output = T>
    + MulAssign
    + Div<Output = T>
    + DivAssign
{
    fn sqrt(&self) -> T;
    fn zero() -> T;
    fn one() -> T;
    fn two() -> T;
    fn splat(val: T) -> T;
    fn abs(&self) -> T;
    fn minimum(&self, other: T) -> T;
    fn maximum(&self, other: T) -> T;
}

macro_rules! impl_math_components_float {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    <$t>::sqrt(*self)
                }

                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1.0
                }

                fn two() -> Self {
                    2.0
                }

                fn splat(num: $t) -> Self {
                    num
                }

                fn minimum(&self, other: $t) -> Self {
                    <$t>::min(*self, other)
                }

                fn maximum(&self, other: $t) -> Self {
                    <$t>::max(*self, other)
                }

                fn abs(&self) -> Self {
                    <$t>::abs(*self)
                }
            }
        )*
    }
}

macro_rules! impl_math_components_integer {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    f32::sqrt(*self as f32) as $t
                }
                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1
                }

                fn two() -> Self {
                    2
                }

                fn splat(val: $t) -> Self {
                    val
                }

                fn minimum(&self, other: $t) -> Self {
                    <$t>::min(*self, other)
                }

                fn maximum(&self, other: $t) -> Self {
                    <$t>::max(*self, other)
                }

                fn abs(&self) -> Self {
                    <$t>::abs(*self)
                }
            }
        )*
    }
}

macro_rules! impl_math_components_unsigned {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    f32::sqrt(*self as f32) as $t
                }
                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1
                }

                fn two() -> Self {
                    2
                }

                fn splat(val: $t) -> Self {
                    val
                }

                fn minimum(&self, other: $t) -> Self {
                    <$t>::min(*self, other)
                }

                fn maximum(&self, other: $t) -> Self {
                    <$t>::max(*self, other)
                }

                fn abs(&self) -> Self {
                    *self
                }
            }
        )*
    }
}

impl_math_components_float!(f32);
impl_math_components_integer!(i8, i16, i32);
impl_math_components_unsigned!(u8, u16, u32);

impl MathComponent<i64> for i64 {
    fn sqrt(&self) -> i64 {
        f64::sqrt(*self as f64) as i64
    }

    fn zero() -> i64 {
        0
    }

    fn one() -> i64 {
        1
    }

    fn two() -> i64 {
        2
    }

    fn splat(val: i64) -> i64 {
        val
    }

    fn abs(&self) -> i64 {
        i64::abs(*self)
    }

    fn minimum(&self, other: i64) -> i64 {
        *self.min(&other)
    }

    fn maximum(&self, other: i64) -> i64 {
        *self.max(&other)
    }
}

impl MathComponent<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }

    fn zero() -> f64 {
        0.0
    }

    fn one() -> f64 {
        1.0
    }

    fn two() -> f64 {
        2.0
    }

    fn splat(val: f64) -> f64 {
        val
    }

    fn abs(&self) -> f64 {
        f64::abs(*self)
    }

    fn minimum(&self, other: f64) -> f64 {
        self.min(other)
    }

    fn maximum(&self, other: f64) -> f64 {
        self.max(other)
    }
}
