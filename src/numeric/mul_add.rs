use crate::numeric::float::Float;

pub trait MulAdd<A = Self, B = Self> {
    type Output;

    fn mul_add(self, a: A, b: B) -> Self::Output;
}

pub trait MullAddAssign<A = Self, B = Self> {
    fn mull_add_assign(&mut self, a: A, b: B);
}

impl MulAdd<f32, f32> for f32 {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as Float>::mul_add(self, a, b)
    }
}

impl MulAdd<f64, f64> for f64 {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        <Self as Float>::mul_add(self, a, b)
    }
}

macro_rules! mul_add_impl {
    ($($t:ty)*) => {$(
        impl MulAdd for $t {
            type Output = Self;

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self::Output {
                (self * a) + b
            }
        }
    )*}
}

mul_add_impl!(isize usize i8 u8 i16 u16 i32 u32 i64 u64);
