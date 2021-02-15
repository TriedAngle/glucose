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
    fn from_i32(num: i32) -> T;
    fn abs(&self) -> T;
    fn minimum(&self, other: T) -> T;
    fn maximum(&self, other: T) -> T;
}

macro_rules! impl_vector_components {
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

                fn from_i32(num: i32) -> Self {
                    num as $t
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

macro_rules! impl_vector_components_not_float {
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

                fn from_i32(num: i32) -> Self {
                    num as $t
                }

                fn minimum(&self, other: $t) -> Self {
                    <$t>::min(*self, other)
                }

                fn maximum(&self, other: $t) -> Self {
                    <$t>::max(*self, other)
                }

                fn abs(&self) -> Self {
                    self.abs()
                }
            }
        )*
    }
}

macro_rules! impl_vector_components_not_float_64 {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    f64::sqrt(*self as f64) as $t
                }
                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1
                }

                fn from_i32(num: i32) -> Self {
                    num as $t
                }

                fn minimum(&self, other: $t) -> Self {
                    <$t>::min(*self, other)
                }

                fn maximum(&self, other: $t) -> Self {
                    <$t>::max(*self, other)
                }

                fn abs(&self) -> Self {
                    self.abs()
                }
            }
        )*
    }
}

impl_vector_components!(f32, f64);
impl_vector_components_not_float!(i8, i16, i32, u8, u16, u32);
impl_vector_components_not_float_64!(i64, u64);
