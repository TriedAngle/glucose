use crate::numeric::num::Num;
use crate::numeric::sign::Signed;
use std::num::FpCategory;
use crate::numeric::trig::Trig;

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
                    Self::is_finite(self) -> bool;
                    Self::is_infinite(self) -> bool;
                    Self::is_nan(self) -> bool;
                    Self::is_normal(self) -> bool;
                    Self::classify(self) -> FpCategory;
                    Self::recip(self) -> Self;
                    Self::floor(self) -> Self;
                    Self::ceil(self) -> Self;
                    Self::round(self) -> Self;
                    Self::trunc(self) -> Self;
                    Self::fract(self) -> Self;
                    Self::copysign(self, sign: Self) -> Self;
                    Self::mul_add(self, a: Self, b: Self) -> Self;
                    Self::div_euclid(self, rhs: Self) -> Self;
                    Self::rem_euclid(self, rhs: Self) -> Self;
                    Self::powi(self, n: i32) -> Self;
                    Self::powf(self, n: Self) -> Self;
                    Self::sqrt(self) -> Self;
                    Self::exp(self) -> Self;
                    Self::exp2(self) -> Self;
                    Self::ln(self) -> Self;
                    Self::log(self, base: Self) -> Self;
                    Self::log2(self) -> Self;
                    Self::log10(self) -> Self;
                    Self::cbrt(self) -> Self;
                    Self::hypot(self, rhs: Self) -> Self;
                    Self::exp_m1(self) -> Self;
                    Self::ln_1p(self) -> Self;
                }
            }
        )*
    }
}

impl_float!(f32 f64);

#[test]
fn test() {
}
