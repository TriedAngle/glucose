use crate::Matrix;

impl<T: Default + Copy, const M: usize> From<(Vec<(T, T)>, Vec<T>, Vec<(T, T)>)>
    for Matrix<T, { M }, { 5 }>
{
    fn from(rhs: (Vec<(T, T)>, Vec<T>, Vec<(T, T)>)) -> Self {
        let mut mat = Self::default();
        mat
    }
}
