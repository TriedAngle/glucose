pub trait Cmp {
    fn minimum(&self, rhs: Self) -> Self;
    fn maximum(&self, rhs: Self) -> Self;
}

macro_rules! impl_cmp {
    ($($t:ty => $min:expr, $max:expr)*) => {
        $(
            impl Cmp for $t {
                fn minimum(&self, rhs: Self) -> Self {
                    ($min)(*self, rhs)
                }

                fn maximum(&self, rhs: Self) -> Self {
                    ($max)(*self, rhs)
                }
            }
        )*
    }
}

impl_cmp! {
    u8  => u8::min,  u8::max
    u16 => u16::min, u16::max
    u32 => u32::min, u32::max
    u64 => u64::min, u64::max
    i8  => i8::min,  i8::max
    i16 => i16::min, i16::max
    i32 => i32::min, i32::max
    i64 => i64::min, i64::max
    f32 => f32::min, f32::max
    f64 => f64::min, f64::max
    u128 => u128::min, u128::max
    i128 => i128::min, i128::max
    usize => usize::min, usize::max
    isize => isize::min, isize::max
}