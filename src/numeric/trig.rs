use crate::forward;
use crate::numeric::num::Num;

pub trait Trig: Sized + Num {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, rhs: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
}

macro_rules! impl_trig_float {
    ($($t:ty)*) => {
        $(
            impl Trig for $t {
                forward! {
                    fn sin(self) -> Self;
                    fn cos(self) -> Self;
                    fn tan(self) -> Self;
                    fn asin(self) -> Self;
                    fn acos(self) -> Self;
                    fn atan(self) -> Self;
                    fn atan2(self, rhs: Self) -> Self;
                    fn sin_cos(self) -> (Self, Self);
                    fn sinh(self) -> Self;
                    fn cosh(self) -> Self;
                    fn tanh(self) -> Self;
                    fn asinh(self) -> Self;
                    fn acosh(self) -> Self;
                    fn atanh(self) -> Self;
                    fn to_degrees(self) -> Self;
                    fn to_radians(self) -> Self;
                }
            }
        )*
    }
}

impl_trig_float!(f32 f64);
