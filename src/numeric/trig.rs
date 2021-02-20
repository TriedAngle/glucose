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
                    Self::sin(self) -> Self;
                    Self::cos(self) -> Self;
                    Self::tan(self) -> Self;
                    Self::asin(self) -> Self;
                    Self::acos(self) -> Self;
                    Self::atan(self) -> Self;
                    Self::atan2(self, rhs: Self) -> Self;
                    Self::sin_cos(self) -> (Self, Self);
                    Self::sinh(self) -> Self;
                    Self::cosh(self) -> Self;
                    Self::tanh(self) -> Self;
                    Self::asinh(self) -> Self;
                    Self::acosh(self) -> Self;
                    Self::atanh(self) -> Self;
                    Self::to_degrees(self) -> Self;
                    Self::to_radians(self) -> Self;
                }
            }
        )*
    }
}

impl_trig_float!(f32 f64);