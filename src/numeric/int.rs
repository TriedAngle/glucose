use crate::numeric::num::Num;
use std::ops::{Not, BitAnd, BitOr, BitXor, Shl, Shr};
use crate::numeric::bit::BitNum;

pub trait Int:
    Num
    + BitNum
    + PartialOrd
    + Ord
    + Eq
    + Copy
{
    fn count_zeros(self) -> u32;
    fn count_ones(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn leading_ones(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
    fn swap_bytes(self) -> Self;
    fn pow(self, exp: u32) -> Self;
}

macro_rules! impl_int {
    ($($t:ty)*) => {
        $(
            impl Int for $t {
                fn count_zeros(self) -> u32 {
                    <$t>::count_zeros(self)
                }

                fn count_ones(self) -> u32 {
                    <$t>::count_ones(self)
                }

                fn leading_zeros(self) -> u32 {
                    <$t>::leading_zeros(self)
                }

                fn trailing_zeros(self) -> u32 {
                    <$t>::trailing_zeros(self)
                }

                fn leading_ones(self) -> u32 {
                    <$t>::leading_ones(self)
                }

                fn trailing_ones(self) -> u32 {
                    <$t>::trailing_ones(self)
                }

                fn rotate_left(self, n: u32) -> Self {
                    <$t>::rotate_left(self, n)
                }

                fn rotate_right(self, n: u32) -> Self {
                    <$t>::rotate_right(self, n)
                }

                fn swap_bytes(self) -> Self {
                    <$t>::swap_bytes(self)
                }

                fn pow(self, exp: u32) -> Self {
                    <$t>::pow(self, exp)
                }

            }
        )*
    }
}

impl_int!(
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    usize isize
);
