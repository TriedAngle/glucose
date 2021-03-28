//! Bivectors and Roters are not abstracted over their dimension because I don't know how
//! I will probably fix this in the future!
mod bivec;
mod dynamic;
mod mat;
mod rotor;
mod scalar;
mod vec;

pub use bivec::*;
pub use dynamic::*;
pub use mat::*;
pub use rotor::*;
pub use scalar::Scalar;
pub use vec::*;
