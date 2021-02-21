use crate::linear::scalar::Scalar;
use crate::linear::vec::Vector;

impl<T: Scalar> Vector<T, 2> {
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] + rhs[0], self[1] + rhs[1]],
        }
    }

    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] - rhs[0], self[1] - rhs[1]],
        }
    }

    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs],
        }
    }

    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs],
        }
    }
}

impl<T: Scalar> Vector<T, 3> {
    pub fn add_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }

    pub fn sub_fast(&self, rhs: Self) -> Self {
        Self {
            data: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }

    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }

    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl<T: Scalar> Vector<T, 4> {
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

    pub fn mul_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs],
        }
    }

    pub fn div_fast(&self, rhs: T) -> Self {
        Self {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs],
        }
    }
}

impl<T: Scalar> Vector<T, 5> {
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
