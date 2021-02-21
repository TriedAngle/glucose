#![allow(unused)]

pub mod linear;
pub mod numeric;

// this allows syntax like:
// impl<T, const N: usize> Vector<T, { N }> where Assert::<N > 3>: IsTrue { }
// TODO: wait for 76560 => restrict ops impls so manual impls can also use ops
// enum Assert<const Check: bool> {}
//
// trait IsTrue {}
//
// impl IsTrue for Assert<true> {}
