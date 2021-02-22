#![allow(unused)]

mod impls;
mod linear;
pub mod numeric;

pub use linear::bivec::Bivector2;
pub use linear::mat::{Matrix, SquareMatrix};
pub use linear::rotor::Rotor2;
pub use linear::scalar::{Scalar, Two};
pub use linear::vec::{Point, Vector};

pub use numeric as num;

// this allows syntax like:
// impl<T, const N: usize> Vector<T, { N }> where Assert::<N > 3>: IsTrue { }
// TODO: wait for 76560 => restrict ops impls so manual impls can also use ops
// enum Assert<const Check: bool> {}
//
// trait IsTrue {}
//
// impl IsTrue for Assert<true> {}
