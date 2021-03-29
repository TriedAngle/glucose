#![allow(unused)]
#![deny(unused_imports)]

#[cfg(feature = "algebra")]
pub mod algebra;
#[cfg(feature = "groups")]
pub mod group_theory;
pub mod impls;

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
