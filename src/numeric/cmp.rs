pub trait Cmp {
    fn minimum(&self, rhs: Self) -> Self;
    fn maximum(&self, rhs: Self) -> Self;
}

macro_rules! impl_cmp {
    ($($t:ty)*) => {
        $(
            impl Cmp for $t {
                fn minimum(&self, rhs: Self) -> Self {
                    <$t>::min(*self, rhs)
                }

                fn maximum(&self, rhs: Self) -> Self {
                    <$t>::max(*self, rhs)
                }
            }
        )*
    }
}

impl_cmp!(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 u128 i128 usize isize);