use crate::numeric::num::Num;
use std::ops::Neg;

pub trait Signed: Sized + Num + Neg<Output = Self> {
    fn abs(&self) -> Self;
    fn abs_sub(&self, rhs: &Self) -> Self;
    fn signum(&self) -> Self;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

macro_rules! impl_signed_int {
    ($($t:ty)*) => {
        $(
            impl Signed for $t {
                #[inline]
                fn abs(&self) -> Self {
                    <$t>::abs(*self)
                }

                #[inline]
                fn abs_sub(&self, rhs: &Self) -> Self {
                    if *self <= *rhs {
                        0
                    } else {
                        *self - *rhs
                    }
                }

                #[inline]
                fn signum(&self) -> Self {
                    <$t>::signum(*self)
                }

                #[inline]
                fn is_positive(&self) -> bool {
                    <$t>::is_positive(*self)
                }

                #[inline]
                fn is_negative(&self) -> bool {
                    <$t>::is_positive(*self)
                }
            }
        )*
    }
}

macro_rules! impl_signed_float {
    ($($t:ty)*) => {
        $(
            impl Signed for $t {
                #[inline]
                fn abs(&self) -> Self {
                    <$t>::abs(*self)
                }

                #[inline]
                fn abs_sub(&self, rhs: &Self) -> Self {
                    if *self <= *rhs {
                        0.0
                    } else {
                        *self - *rhs
                    }
                }

                #[inline]
                fn signum(&self) -> Self {
                    <$t>::signum(*self)
                }

                #[inline]
                fn is_positive(&self) -> bool {
                    <$t>::is_sign_positive(*self)
                }

                #[inline]
                fn is_negative(&self) -> bool {
                    <$t>::is_sign_positive(*self)
                }
            }
        )*
    }
}

impl_signed_int!(isize i8 i16 i32 i64 i128);
impl_signed_float!(f32 f64);
