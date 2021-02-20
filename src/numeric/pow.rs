pub trait Pow<Rhs> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_pow {
    ($($t:ty => [$method:expr => $rhs:ty | $from:ty]);+) => {
        $(
            impl Pow<$rhs> for $t {
                type Output = $t;

                #[inline]
                fn pow(self, rhs: $rhs) -> $t {
                    ($method)(self, <$from>::from(rhs))
                }
            }
        )*
    }
}

impl_pow! {
    u8  => [u8::pow  => u8  | u32];
    u8  => [u8::pow  => u16 | u32];
    u8  => [u8::pow  => u32 | u32];
    u16 => [u16::pow => u8  | u32];
    u16 => [u16::pow => u16 | u32];
    u16 => [u16::pow => u32 | u32];
    u32 => [u32::pow => u8  | u32];
    u32 => [u32::pow => u16 | u32];
    u32 => [u32::pow => u32 | u32];
    u64 => [u64::pow => u8  | u32];
    u64 => [u64::pow => u16 | u32];
    u64 => [u64::pow => u32 | u32];
    i8  => [i8::pow  => u8  | u32];
    i8  => [i8::pow  => u16 | u32];
    i8  => [i8::pow  => u32 | u32];
    i16 => [i16::pow => u8  | u32];
    i16 => [i16::pow => u16 | u32];
    i16 => [i16::pow => u32 | u32];
    i32 => [i32::pow => u8  | u32];
    i32 => [i32::pow => u16 | u32];
    i32 => [i32::pow => u32 | u32];
    i64 => [i64::pow => u8  | u32];
    i64 => [i64::pow => u16 | u32];
    i64 => [i64::pow => u32 | u32]
}

impl_pow! {
    usize => [usize::pow => u8  | u32];
    usize => [usize::pow => u16 | u32];
    usize => [usize::pow => u32 | u32];
    isize => [isize::pow => u8  | u32];
    isize => [isize::pow => u16 | u32];
    isize => [isize::pow => u32 | u32]
}

impl_pow! {
    u128 => [u128::pow => u8  | u32];
    u128 => [u128::pow => u16 | u32];
    u128 => [u128::pow => u32 | u32];
    i128 => [i128::pow => u8  | u32];
    i128 => [i128::pow => u16 | u32];
    i128 => [i128::pow => u32 | u32]
}