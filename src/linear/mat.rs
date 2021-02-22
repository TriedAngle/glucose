use crate::linear::scalar::Scalar;
use crate::linear::vec::Vector;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::alloc::Layout;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [Vector<T, { M }>; N],
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, { M }, { N }> {
    #[inline]
    fn default() -> Self {
        Self {
            data: [Vector::default(); N],
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

    pub const fn new(data: [Vector<T, { M }>; N]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        M * N
    }

    #[inline]
    pub fn layout() -> Layout {
        Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<[T; M]>())
            .unwrap()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // this is safe because the underlying data structure of a matrix has length M * N
        unsafe { std::slice::from_raw_parts(self as *const Self as *const T, M * N) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut T, M * N) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const Self as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Self as *mut T
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                M * N * std::mem::size_of::<Self>(),
            )
        }
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn to_vectors(&self) -> [Vector<T, { M }>; N] {
        self.data
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [Vector::zero(); N],
        }
    }
}

impl<T: Scalar, const N: usize> Matrix<T, { N }, { N }> {
    #[inline]
    pub fn new_identity() -> Self {
        let mut mat = Matrix::default();
        for i in 0..N {
            mat.data[i] = Vector::unit(N);
        }
        mat
    }
}

impl<T: Scalar, const M: usize, const N: usize> Add for Matrix<T, { M }, { N }> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::default(); N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Sub for Matrix<T, { M }, { N }> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::default(); N];
        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        Self { data }
    }
}

impl<T: Scalar, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, { N }, { P }>>
    for Matrix<T, { M }, { N }>
{
    type Output = Matrix<T, { M }, { P }>;
    #[inline]
    fn mul(self, rhs: Matrix<T, { N }, { P }>) -> Self::Output {
        let mut data = [Vector::default(); P];
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

impl<T: Scalar, const N: usize, const M: usize> Mul<Vector<T, { N }>> for Matrix<T, { M }, { N }> {
    type Output = Vector<T, { M }>;

    #[inline]
    fn mul(self, rhs: Vector<T, { N }>) -> Self::Output {
        Vector::from(self * Matrix::from(rhs))
    }
}

impl<T, const M: usize> From<Vector<T, { M }>> for Matrix<T, { M }, 1> {
    #[inline]
    fn from(rhs: Vector<T, { M }>) -> Self {
        let data = [rhs];
        Self { data }
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> From<[[T; M]; N]>
    for Matrix<T, { M }, { N }>
{
    #[inline]
    fn from(rhs: [[T; M]; N]) -> Self {
        let mut mat = Matrix::default();
        for i in 0..N {
            mat.data[i].data = rhs[i];
        }
        mat
    }
}
