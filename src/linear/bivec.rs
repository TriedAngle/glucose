use crate::linear::scalar::Scalar;
use crate::numeric::float::Float;
use crate::numeric::identity::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bivector2<T> {
    pub data: T,
}

impl<T> Bivector2<T> {
    pub const fn new(data: T) -> Self {
        Self { data }
    }

    #[inline]
    pub fn layout() -> std::alloc::Layout {
        std::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<T>())
            .unwrap()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const T, 1) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut T, 1) }
    }

    #[inline]
    pub fn as_byte_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<T>())
        }
    }

    #[inline]
    pub fn as_mut_byte_slice(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, std::mem::size_of::<T>())
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const Self as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Self as *mut T
    }
}

impl<T: Scalar> Bivector2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.data * rhs.data
    }
}

impl<T: Scalar + Float> Bivector2<T> {
    pub fn magnitude_squared(&self) -> T {
        self.data * self.data
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.data /= mag;
    }

    pub fn normalized(&self) -> Self {
        let mut bivec = *self;
        bivec.normalize();
        bivec
    }
}

impl<T: Scalar> Add for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Bivector2<T>) -> Self {
        self += rhs;
        self
    }
}

impl<T: Scalar> AddAssign for Bivector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Bivector2<T>) {
        self.data += rhs.data;
    }
}

impl<T: Scalar> Sub for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Bivector2<T>) -> Self {
        self -= rhs;
        self
    }
}

impl<T: Scalar> SubAssign for Bivector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Bivector2<T>) {
        self.data -= rhs.data;
    }
}

impl<T: Scalar> Mul for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: Bivector2<T>) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Scalar> Mul<T> for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Scalar> MulAssign for Bivector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data;
    }
}

impl<T: Scalar> MulAssign<T> for Bivector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.data *= rhs;
    }
}

impl<T: Scalar> Div for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: Bivector2<T>) -> Self {
        self /= rhs;
        self
    }
}

impl<T: Scalar> Div<T> for Bivector2<T> {
    type Output = Bivector2<T>;
    #[inline]
    fn div(mut self, rhs: T) -> Bivector2<T> {
        self.data /= rhs;
        self
    }
}

impl<T: Scalar> DivAssign for Bivector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.data /= rhs.data;
    }
}

impl<T: Scalar> DivAssign<T> for Bivector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.data /= rhs;
    }
}

impl<T: Scalar> Neg for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self {
        self.data = -self.data;
        self
    }
}

impl<T: Scalar> Zero for Bivector2<T> {
    fn zero() -> Self {
        Self { data: <T>::zero() }
    }

    fn is_zero(&self) -> bool {
        self.data == <T>::zero()
    }
}

impl<T: Scalar> One for Bivector2<T> {
    fn one() -> Self {
        Self { data: <T>::one() }
    }

    fn is_one(&self) -> bool {
        self.data == <T>::one()
    }
}
