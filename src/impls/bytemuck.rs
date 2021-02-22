use crate::*;
use bytemuck::{Pod, Zeroable};

unsafe impl<T: Copy, const N: usize> Pod for Vector<T, { N }> {}
unsafe impl<T, const N: usize> Zeroable for Vector<T, { N }> {}

unsafe impl<T: Copy, const M: usize, const N: usize> Pod for Matrix<T, { M }, { N }> {}
unsafe impl<T, const M: usize, const N: usize> Zeroable for Matrix<T, { M }, { N }> {}

unsafe impl<T: Copy, const N: usize> Pod for Bivector2<T> {}
unsafe impl<T, const N: usize> Zeroable for Bivector2<T> {}

unsafe impl<T: Copy, const N: usize> Pod for Rotor2<T> {}
unsafe impl<T, const N: usize> Zeroable for Rotor2<T> {}
