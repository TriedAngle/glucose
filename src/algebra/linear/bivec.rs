use crate::algebra::linear::scalar::Scalar;
use fructose::algebra::helpers::identity::{One, Zero};
use fructose::algebra::properties::general::{
    Associative, Commutative, Identity, Invertible, PartiallyOrdered, Set, Total,
};
use fructose::operators::{
    Additive, ClosedAdd, ClosedDiv, ClosedMul, ClosedNeg, ClosedSub, Multiplicative,
};
use fructose::specific::complex::Real;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
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

impl<T: Scalar + ClosedMul> Bivector2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.data * rhs.data
    }
}

impl<T: Scalar + Real + ClosedDiv + ClosedMul> Bivector2<T> {
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

impl<T: Scalar + ClosedAdd> Add for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Bivector2<T>) -> Self {
        self += rhs;
        self
    }
}

impl<T: Scalar + ClosedAdd> AddAssign for Bivector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Bivector2<T>) {
        self.data += rhs.data;
    }
}

impl<T: Scalar + ClosedSub> Sub for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Bivector2<T>) -> Self {
        self -= rhs;
        self
    }
}

impl<T: Scalar + ClosedSub> SubAssign for Bivector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Bivector2<T>) {
        self.data -= rhs.data;
    }
}

impl<T: Scalar + ClosedMul> Mul for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: Bivector2<T>) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Scalar + ClosedMul> Mul<T> for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}

impl<T: Scalar + ClosedMul> MulAssign for Bivector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data;
    }
}

impl<T: Scalar + ClosedMul> MulAssign<T> for Bivector2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.data *= rhs;
    }
}

impl<T: Scalar + ClosedDiv> Div for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: Bivector2<T>) -> Self {
        self /= rhs;
        self
    }
}

impl<T: Scalar + ClosedDiv> Div<T> for Bivector2<T> {
    type Output = Bivector2<T>;
    #[inline]
    fn div(mut self, rhs: T) -> Bivector2<T> {
        self.data /= rhs;
        self
    }
}

impl<T: Scalar + ClosedDiv> DivAssign for Bivector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.data /= rhs.data;
    }
}

impl<T: Scalar + ClosedDiv> DivAssign<T> for Bivector2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.data /= rhs;
    }
}

impl<T: Scalar + ClosedNeg> Neg for Bivector2<T> {
    type Output = Self;
    #[inline]
    fn neg(mut self) -> Self {
        self.data = -self.data;
        self
    }
}

impl<T: Scalar + ClosedAdd> Set<Additive> for Bivector2<T> {
    fn operate(&self, rhs: Self) -> Self {
        *self + rhs
    }
}

impl<T: Scalar + ClosedMul> Set<Multiplicative> for Bivector2<T> {
    fn operate(&self, rhs: Self) -> Self {
        *self * rhs
    }
}

impl<T: Scalar + ClosedAdd + Total<Additive>> Total<Additive> for Bivector2<T> {}

impl<T: Scalar + ClosedAdd + Associative<Additive>> Associative<Additive> for Bivector2<T> {}

impl<T: Scalar + ClosedAdd + Commutative<Additive>> Commutative<Additive> for Bivector2<T> {}

impl<T: Scalar + ClosedMul + Total<Multiplicative>> Total<Multiplicative> for Bivector2<T> {}

impl<T: Scalar + ClosedMul + Associative<Multiplicative>> Associative<Multiplicative>
    for Bivector2<T>
{
}

impl<T: Scalar + ClosedMul + Commutative<Multiplicative>> Commutative<Multiplicative>
    for Bivector2<T>
{
}

impl<T: Scalar + Identity<Additive> + ClosedAdd> Identity<Additive> for Bivector2<T> {
    fn identity() -> Self {
        Self::new(T::identity())
    }

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}

impl<T: Scalar + Identity<Multiplicative> + ClosedMul> Identity<Multiplicative> for Bivector2<T> {
    fn identity() -> Self {
        Self::new(T::identity())
    }

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}

impl<T: Scalar + PartiallyOrdered<Additive>> PartiallyOrdered<Additive> for Bivector2<T> {}

impl<T: Scalar + PartiallyOrdered<Multiplicative>> PartiallyOrdered<Multiplicative>
    for Bivector2<T>
{
}

impl<T: Scalar + ClosedAdd + Identity<Additive> + PartiallyOrdered<Additive>> Zero
    for Bivector2<T>
{
    fn zero() -> Self {
        Self {
            data: <T>::identity(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T: Scalar + ClosedMul + Identity<Multiplicative> + PartiallyOrdered<Multiplicative>> One
    for Bivector2<T>
{
    fn one() -> Self {
        Self {
            data: <T>::identity(),
        }
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

impl<T: Scalar + ClosedAdd + Invertible<Additive>> Invertible<Additive> for Bivector2<T> {
    fn inverse(&self) -> Self {
        -*self
    }

    fn inverted(&mut self) {
        *self = -*self;
    }
}

impl<T: Scalar + ClosedMul + Invertible<Multiplicative>> Invertible<Multiplicative>
    for Bivector2<T>
{
    fn inverse(&self) -> Self {
        unimplemented!()
    }

    fn inverted(&mut self) {
        *self = -*self;
    }
}
