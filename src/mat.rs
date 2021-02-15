#![allow(dead_code)]

use crate::traits::MathComponent;
use crate::vec::Vector;
use std::ops::{Add, Mul, Sub};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        M * N
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn zero() -> Self {
        Self {
            data: [[<T>::zero(); N]; M],
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SquareMatrix<T, { N }> {
    pub fn new_identity() -> Self {
        let mut data = [[<T>::zero(); N]; N];
        for i in 0..N {
            data[i][i] = <T>::one();
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Add for Matrix<T, { M }, { N }> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[<T>::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Sub for Matrix<T, { M }, { N }> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[<T>::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, { N }, { P }>>
    for Matrix<T, { M }, { N }>
where
    T: MathComponent<T> + Copy,
{
    type Output = Matrix<T, { M }, { P }>;

    fn mul(self, rhs: Matrix<T, { N }, { P }>) -> Self::Output {
        unimplemented!()
    }
}

impl<T: Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn to_vectors(&self) -> Vec<Vector<T, { N }>> {
        self.data.iter().map(|col| Vector::new(*col)).collect()
    }
}
