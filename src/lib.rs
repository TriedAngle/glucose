#![allow(unused)]
#![deny(unused_imports)]

pub mod algebra;
pub mod group_theory;
pub mod impls;
// pub mod number_theory;

#[macro_use]
pub mod util;
// this allows syntax like:
// impl<T, const N: usize> Vector<T, { N }> where Assert::<N > 3>: IsTrue { }
// TODO: wait for 76560 => restrict ops impls so manual impls can also use ops
// enum Assert<const Check: bool> {}
//
// trait IsTrue {}
//
// impl IsTrue for Assert<true> {}
