use crate::numeric::num::NumAssign;
use std::ops::Neg;

pub trait Scalar: NumAssign + Default + Copy + Neg<Output = Self> + PartialEq {}

impl<T: NumAssign + Default + Copy + Neg<Output = Self> + PartialEq> Scalar for T {}

// I don't like having this trait but this is the easiest fix I can think of right now.
// this trait will be removed or replaced probably sooner or later
pub trait Two: Sized {
    fn two() -> Self;

    fn set_two(&mut self) {
        *self = Self::two()
    }

    fn is_two(&self) -> bool;
}

macro_rules! impl_two {
    ($($t:ty => $two:expr)*) => {
        $(
            impl Two for $t {
                fn two() -> Self {
                    $two
                }

                fn is_two(&self) -> bool {
                    *self == $two
                }
            }
        )*
    }
}

impl_two! {
    u8  => 2
    u16 => 2
    u32 => 2
    u64 => 2
    i8  => 2
    i16 => 2
    i32 => 2
    i64 => 2
    u128 => 2
    i128 => 2
    usize => 2
    isize => 2
    f32 => 2.0
    f64 => 2.0
}
