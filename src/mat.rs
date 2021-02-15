#![allow(dead_code)]

use crate::vec::Vector;
use crate::traits::MathComponent;

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[derive(Debug)]
pub struct Matrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N]
}

impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn new(data: [[T; M]; N]) -> Self {
        Self {
            data,
        }
    }

    pub fn len(&self) -> usize {
        M * N
    }
}

impl<T: MathComponent<T> + Copy, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn zero() -> Self {
        Self {
            data: [[<T>::zero(); M]; N]
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SquareMatrix<T, { N }> {
    pub fn new_identity() -> Self {
        let mut data = [[<T>::zero(); N]; N];
        for i in 0..N {
            data[i][i] = <T>::one();
        }
        Self {
            data
        }
    }
}

impl<T: Copy, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn to_vectors(&self) -> Vec<Vector<T, { M }>> {
        self.data.iter().map(|col| Vector::new(*col)).collect()
    }
}

#[test]
fn test() {
    let mat = Matrix::<f32, 3, 3>::new_identity();
    println!("{:?}", mat);
}
