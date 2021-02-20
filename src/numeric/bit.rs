use crate::numeric::num::Num;
use std::ops::{BitAnd, Not, BitXor, BitOr, Shl, Shr, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use crate::numeric::identity::{One, Zero};

pub trait BitNum: PartialEq + Zero + One + BitOps {}

pub trait BitOps<Rhs = Self, Output = Self>:
    BitAnd<Rhs, Output = Output>
    + BitOr<Rhs, Output = Output>
    + BitXor<Rhs, Output = Output>
    + Not<Output = Output>
    + Shl<Rhs, Output = Output>
    + Shr<Rhs, Output = Output>
{
}

pub trait BitAssignOps<Rhs = Self>:
    BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs> + ShlAssign<Rhs> + ShrAssign<Rhs>
{
}

pub trait BitAssign: BitNum + BitAssignOps {}

impl<T> BitNum for T where T: Num + BitOps {}

impl<T, Rhs, Output> BitOps<Rhs, Output> for T where
    T: BitAnd<Rhs, Output = Output>
        + BitOr<Rhs, Output = Output>
        + BitXor<Rhs, Output = Output>
        + Not<Output = Output>
        + Shl<Rhs, Output = Output>
        + Shr<Rhs, Output = Output>
{
}

impl<T, Rhs> BitAssignOps<Rhs> for T where
    T: BitAndAssign<Rhs> + BitOrAssign<Rhs> + BitXorAssign<Rhs> + ShlAssign<Rhs> + ShrAssign<Rhs>
{
}

impl<T> BitAssign for T where T: BitNum + BitAssignOps {}