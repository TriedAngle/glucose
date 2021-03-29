#[cfg(feature = "algebra")]
pub mod algebra {
    use crate::algebra::linear::{Bivector2, Matrix, Rotor2};
    use bytemuck::{Pod, Zeroable};
    unsafe impl<T: Pod, const M: usize, const N: usize> Pod for Matrix<T, { M }, { N }> {}
    unsafe impl<T: Zeroable, const M: usize, const N: usize> Zeroable for Matrix<T, { M }, { N }> {}

    unsafe impl<T: Pod> Pod for Bivector2<T> {}
    unsafe impl<T: Zeroable> Zeroable for Bivector2<T> {}

    unsafe impl<T: Pod> Pod for Rotor2<T> {}
    unsafe impl<T: Zeroable> Zeroable for Rotor2<T> {}
}
