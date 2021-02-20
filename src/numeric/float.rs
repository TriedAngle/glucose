use crate::numeric::num::Num;
use crate::numeric::sign::Signed;
use crate::numeric::trig::Trig;
use std::num::FpCategory;

pub trait Float: Num + Signed + Trig + PartialOrd + Copy {
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_sing_negative(self) -> bool;
    fn classify(self) -> FpCategory;
    fn recip(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn copysign(self, sign: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, rhs: Self) -> Self;
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
}

macro_rules! impl_float {
    ($($t:ty)*) => {
        $(
            impl Float for $t {

                fn is_sign_positive(self) -> bool {
                    <$t>::is_sign_positive(self)
                }
                fn is_sing_negative(self) -> bool{
                    <$t>::is_sign_negative(self)
                }

                forward! {
                    fn is_finite(self) -> bool;
                    fn is_infinite(self) -> bool;
                    fn is_nan(self) -> bool;
                    fn is_normal(self) -> bool;
                    fn classify(self) -> FpCategory;
                    fn recip(self) -> Self;
                    fn floor(self) -> Self;
                    fn ceil(self) -> Self;
                    fn round(self) -> Self;
                    fn trunc(self) -> Self;
                    fn fract(self) -> Self;
                    fn copysign(self, sign: Self) -> Self;
                    fn mul_add(self, a: Self, b: Self) -> Self;
                    fn div_euclid(self, rhs: Self) -> Self;
                    fn rem_euclid(self, rhs: Self) -> Self;
                    fn powi(self, n: i32) -> Self;
                    fn powf(self, n: Self) -> Self;
                    fn sqrt(self) -> Self;
                    fn exp(self) -> Self;
                    fn exp2(self) -> Self;
                    fn ln(self) -> Self;
                    fn log(self, base: Self) -> Self;
                    fn log2(self) -> Self;
                    fn log10(self) -> Self;
                    fn cbrt(self) -> Self;
                    fn hypot(self, rhs: Self) -> Self;
                    fn exp_m1(self) -> Self;
                    fn ln_1p(self) -> Self;
                }
            }
        )*
    }
}

impl_float!(f32 f64);