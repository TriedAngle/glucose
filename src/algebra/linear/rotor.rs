use crate::algebra::linear::bivec::Bivector2;
use crate::algebra::linear::mat::Matrix;
use crate::algebra::linear::scalar::{Scalar, Two};
use crate::algebra::linear::vec::Vector;
use fructose::algebra::helpers::identity::{One, Zero};
use fructose::algebra::helpers::mul_add::MulAdd;
use fructose::algebra::helpers::trig::TrigOps;
use fructose::algebra::properties::general::{Associative, Commutative, Identity, Set, Total};
use fructose::operators::{
    Additive, ClosedAdd, ClosedDiv, ClosedMul, ClosedNeg, ClosedOps, Multiplicative,
};
use fructose::specific::complex::Real;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotor2<S> {
    pub scalar: S,
    pub bivector: Bivector2<S>,
}

impl<S> Rotor2<S> {
    #[inline]
    pub const fn new(scalar: S, bivector: Bivector2<S>) -> Self {
        Self { scalar, bivector }
    }

    #[inline]
    pub fn layout() -> std::alloc::Layout {
        std::alloc::Layout::from_size_align(std::mem::size_of::<Self>(), std::mem::align_of::<S>())
            .unwrap()
    }
}

impl<S: Scalar + One + Zero + ClosedOps + ClosedNeg> Rotor2<S> {
    #[inline]
    pub fn identity() -> Self {
        Self {
            scalar: <S>::one(),
            bivector: Bivector2::zero(),
        }
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.bivector = -self.bivector;
    }

    #[inline]
    pub fn reversed(&self) -> Self {
        let mut s = *self;
        s.reverse();
        s
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> S {
        self.scalar * rhs.scalar + self.bivector.dot(rhs.bivector)
    }
}

impl<S: Scalar + ClosedOps + MulAdd<Output = S>> Rotor2<S> {
    #[inline]
    pub fn rotate_by(&mut self, other: Self) {
        let b = *self;
        let a = other;
        let sa2_plus_baxy2 = a
            .scalar
            .mul_add(a.scalar, a.bivector.data * a.bivector.data);

        self.scalar =
            (a.scalar - b.scalar) * a.bivector.data * b.bivector.data + b.scalar * sa2_plus_baxy2;
        self.bivector.data = b.bivector.data * sa2_plus_baxy2;
    }

    #[inline]
    pub fn rotated_by(mut self, other: Self) -> Self {
        self.rotate_by(other);
        self
    }
}

impl<S: Scalar + ClosedOps + Two + ClosedNeg> Rotor2<S> {
    #[inline]
    pub fn rotate_vec(self, vec: &mut Vector<S, 2>) {
        let fx = self.scalar * vec[0] + self.bivector.data * vec[1];
        let fy = self.scalar * vec[1] - (self.bivector.data * vec[0]);

        vec[0] = self.scalar * fx + self.bivector.data * fy;
        vec[1] = self.scalar * fy - (self.bivector.data * fx);
    }

    #[inline]
    pub fn into_matrix(self) -> Matrix<S, 2, 2> {
        let s2_minus_bxy2 = self.scalar * self.scalar - self.bivector.data * self.bivector.data;
        let two_s_bxy = S::two() * self.scalar * self.bivector.data;

        Matrix::new([[s2_minus_bxy2, -two_s_bxy], [two_s_bxy, s2_minus_bxy2]])
    }
}

impl<S: Scalar + ClosedAdd> Add for Rotor2<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.scalar + rhs.scalar, self.bivector + rhs.bivector)
    }
}

impl<S: Scalar + ClosedAdd> AddAssign for Rotor2<S> {
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += self.scalar;
        self.bivector += rhs.bivector;
    }
}

impl<S: Scalar + ClosedMul + ClosedAdd + Real> Rotor2<S> {
    // #[inline]
    // pub fn from_rotation_between(from: Vector<S, 2>, rhs: Vector<S, 2>) -> Self {
    //     Self::new(<S>::one() + rhs.dot(from), rhs.wedge(from)).normalized()
    // }

    // #[inline]
    // pub fn magnitude_squared(&self) -> S {
    //     self.scalar * self.scalar + self.bivector.magnitude_squared()
    // }
    // #[inline]
    // pub fn magnitude(&self) -> S {
    //     self.magnitude_squared().sqrt()
    // }
    //
    // #[inline]
    // pub fn normalize(&mut self) {
    //     let mag = self.magnitude();
    //     self.scalar /= mag;
    //     self.bivector.data /= mag;
    // }
    //
    // #[inline]
    // pub fn normalized(&self) -> Self {
    //     let mut rotor = *self;
    //     rotor.normalize();
    //     rotor
    // }
}

impl<S: Scalar + Two + Real + ClosedDiv + TrigOps> Rotor2<S> {
    #[inline]
    pub fn from_angle(angle: S) -> Self {
        let half_angle = angle / <S>::two();
        let (sin, cos) = half_angle.sin_cos();
        Self::new(cos, Bivector2::new(-sin))
    }
}

impl<T: Scalar + ClosedAdd> Set<Additive> for Rotor2<T> {
    fn operate(&self, rhs: Self) -> Self {
        *self + rhs
    }
}

impl<T: Scalar + ClosedAdd + Total<Additive>> Total<Additive> for Rotor2<T> {}

impl<T: Scalar + ClosedAdd + Associative<Additive>> Associative<Additive> for Rotor2<T> {}

impl<T: Scalar + Identity<Additive> + ClosedAdd> Identity<Additive> for Rotor2<T> {
    fn identity() -> Self {
        Self::new(T::identity(), Bivector2::identity())
    }

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}
