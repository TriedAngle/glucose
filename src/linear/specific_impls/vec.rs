use crate::linear::scalar::Scalar;
use crate::linear::vec::Vector;
use crate::linear::bivec::Bivector2;

impl<T: Scalar> Vector<T, 2> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] + rhs[0], self[1] + rhs[1]],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] - rhs[0], self[1] - rhs[1]],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs],
        }
    }

    #[inline]
    pub fn dot_fast(&self, rhs: Self) -> T {
        (self[0] * rhs[0]) + (self[1] * rhs[0])
    }

    #[inline]
    pub fn wedge(&self, other: Self) -> Bivector2<T> {
        Bivector2 { data: (self[0] * other[1]) - (other[0] * self[1]) }
    }
}

impl<T: Scalar> Vector<T, 3> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }

    #[inline]
    pub fn dot_fast(&self, other: Self) -> T {
        (self[0] * other[0]) + (self[1] * other[1]) + (self[2] * other[2])
    }
}

impl<T: Scalar> Vector<T, 4> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
            ],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
            ],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs],
        }
    }
}

impl<T: Scalar> Vector<T, 5> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
                self[4] + rhs[4],
            ],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
                self[4] - rhs[4],
            ],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
                self[3] * rhs,
                self[4] * rhs,
            ],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
                self[3] / rhs,
                self[4] / rhs,
            ],
        }
    }
}

impl<T: Scalar> Vector<T, 6> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
                self[4] + rhs[4],
                self[5] + rhs[5],
            ],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
                self[4] - rhs[4],
                self[5] - rhs[5],
            ],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
                self[3] * rhs,
                self[4] * rhs,
                self[5] * rhs,
            ],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
                self[3] / rhs,
                self[4] / rhs,
                self[5] / rhs,
            ],
        }
    }
}

impl<T: Scalar> Vector<T, 7> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
                self[4] + rhs[4],
                self[5] + rhs[5],
                self[6] + rhs[6],
            ],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
                self[4] - rhs[4],
                self[5] - rhs[5],
                self[6] - rhs[6],
            ],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
                self[3] * rhs,
                self[4] * rhs,
                self[5] * rhs,
                self[6] * rhs,
            ],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
                self[3] / rhs,
                self[4] / rhs,
                self[5] / rhs,
                self[6] / rhs,
            ],
        }
    }
}

impl<T: Scalar> Vector<T, 8> {
    #[inline]
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3],
                self[4] + rhs[4],
                self[5] + rhs[5],
                self[6] + rhs[6],
                self[7] + rhs[7],
            ],
        }
    }

    #[inline]
    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3],
                self[4] - rhs[4],
                self[5] - rhs[5],
                self[6] - rhs[6],
                self[7] - rhs[7],
            ],
        }
    }

    #[inline]
    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
                self[3] * rhs,
                self[4] * rhs,
                self[5] * rhs,
                self[6] * rhs,
                self[7] * rhs,
            ],
        }
    }

    #[inline]
    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
                self[3] / rhs,
                self[4] / rhs,
                self[5] / rhs,
                self[6] / rhs,
                self[7] / rhs,
            ],
        }
    }
}
