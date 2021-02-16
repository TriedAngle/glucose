#![allow(dead_code)]

use crate::traits::MathComponent;
use crate::vec::Vector;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, { M }, { N }> {
    #[inline]
    fn default() -> Self {
        Self {
            data: [[<T>::default(); M]; N],
        }
    }
}

impl<T: Display, const M: usize, const N: usize> Display for Matrix<T, { M }, { N }> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for m in 0..M {
            &string.push_str("|");
            for n in 0..N {
                if n == N - 1 {
                    &string.push_str(&format!("{}", self.data[n][m]));
                    break;
                }
                &string.push_str(&format!("{} ", self.data[n][m]));
            }
            &string.push_str("|\n");
        }
        write!(f, "{}", string)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }
    #[inline]
    pub fn len(&self) -> usize {
        M * N
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn to_vectors(&self) -> [Vector<T, { M }>; N] {
        let mut vec_arr = [Vector::default(); N];
        for i in 0..N {
            vec_arr[i] = self.data[i].into()
        }
        vec_arr
    }
}

impl<T: MathComponent<T> + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [[<T>::zero(); M]; N],
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SquareMatrix<T, { N }> {
    #[inline]
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
    #[inline]
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
    #[inline]
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

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, { N }, { P }>>
for Matrix<T, { M }, { N }>
    where
        T: MathComponent<T> + Copy,
{
    type Output = Matrix<T, { M }, { P }>;
    #[inline]
    fn mul(self, rhs: Matrix<T, { N }, { P }>) -> Self::Output {
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

impl<T: MathComponent<T> + Copy, const N: usize, const M: usize> Mul<Vector<T, { N }>> for Matrix<T, { M }, { N }> {
    type Output = Vector<T, { M }>;

    #[inline]
    fn mul(self, rhs: Vector<T, { N }>) -> Self::Output {
        Vector::from(self * Matrix::from(rhs))
    }
}

impl<T, const M: usize> From<Vector<T, { M }>> for Matrix<T, { M }, 1> {
    #[inline]
    fn from(rhs: Vector<T, { M }>) -> Self {
        let data = [rhs.data; 1];
        Self { data }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; M]; N]> for Matrix<T, { M }, { N }> {
    #[inline]
    fn from(rhs: [[T; M]; N]) -> Self {
        Self { data: rhs }
    }
}