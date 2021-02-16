#![allow(dead_code)]

use crate::traits::MathComponent;
use crate::vec::Vector;
use std::ops::{Add, Mul, Sub};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

impl<T, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        M * N
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn zero() -> Self {
        Self {
            data: [[<T>::zero(); M]; N],
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
        let mut data = [[<T>::default(); M]; N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Sub for Matrix<T, { M }, { N }> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[<T>::default(); M]; N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, { N }, { P }>> for Matrix<T, { M }, { N }>
where
    T: MathComponent<T> + Copy,
{
    type Output = Matrix<T, { M }, { P }>;

    fn mul<>(self, rhs: Matrix<T, { N }, { P }>) -> Self::Output {
        let mut data = [[<T>::default(); M]; P];
        for m in 0..M {
            for p in 0..P {
                for n in 0..N {
                    data[p][m] += self.data[n][m] * rhs.data[p][n];
                }
            }
        }
        Matrix { data }
    }
}

impl<T: Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn to_vectors(&self) -> Vec<Vector<T, { M }>> {
        self.data.iter().map(|col| Vector::new(*col)).collect()
    }
}

impl<T, const M: usize, const N: usize> From<[[T; M]; N]> for Matrix<T, { M }, { N }> {
    fn from(rhs: [[T; M]; N]) -> Self {
        Self { data: rhs }
    }
}

#[test]
fn test() {
    let mat_arr1 = [[1.0, 2.0, 3.0],[2.0, 1.0, 3.0]];
    let mat_arr2 = [[2.0, 1.0], [3.0, 1.0], [2.0, 3.0]];
    let mat1: Matrix<f64, 3, 2> = Matrix::from(mat_arr1);
    let mat2 : Matrix<f64, 2, 3>= Matrix::from(mat_arr2);
    let mat3 =  mat1 * mat2;
    println!("{:?}", mat1);
    println!("{:?}", mat2);
    println!("{:?}", mat3);
}