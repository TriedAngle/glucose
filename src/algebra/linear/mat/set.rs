use super::Matrix;
use crate::algebra::linear::scalar::Scalar;
use crate::algebra::linear::vec::Vector;
use crate::linear::mat::SquareMatrix;
use fructose::algebra::field::{ComplexField, Field};
use fructose::algebra::helpers::identity::{One, Zero};
use fructose::algebra::linear::vector::{AffineSpace, InnerSpace, Norm, NormedSpace, VectorSpace};
use fructose::algebra::module::Module;
use fructose::algebra::properties::general::{
    Associative, Commutative, Identity, Invertible, Set, Total,
};
use fructose::algebra::ring::CommutativeRing;
use fructose::operators::{Additive, ClosedAdd, ClosedMul, ClosedOps, Multiplicative};
use std::iter::Sum;

impl<T: Scalar + ClosedAdd, const M: usize, const N: usize> Set<Additive>
    for Matrix<T, { M }, { N }>
{
    fn operate(&self, rhs: Self) -> Self {
        *self + rhs
    }
}

impl<T: Scalar + ClosedAdd + Total<Additive>, const M: usize, const N: usize> Total<Additive>
    for Matrix<T, { M }, { N }>
{
}

impl<T: Scalar + ClosedAdd + Associative<Additive>, const M: usize, const N: usize>
    Associative<Additive> for Matrix<T, { M }, { N }>
{
}

impl<T: Scalar + ClosedAdd + Commutative<Additive>, const M: usize, const N: usize>
    Commutative<Additive> for Matrix<T, { M }, { N }>
{
}

impl<T: Scalar + ClosedAdd + Identity<Additive> + PartialEq, const M: usize, const N: usize>
    Identity<Additive> for Matrix<T, { M }, { N }>
{
    fn identity() -> Self {
        Self::broadcast(T::identity())
    }

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}

impl<T: Scalar + ClosedAdd + Invertible<Additive>, const M: usize, const N: usize>
    Invertible<Additive> for Matrix<T, { M }, { N }>
{
    fn inverse(&self) -> Self {
        -*self
    }

    fn inverted(&mut self) {
        *self = -*self;
    }
}

impl<T: Scalar + ClosedAdd + ClosedMul, const N: usize> Set<Multiplicative>
    for SquareMatrix<T, { N }>
{
    fn operate(&self, rhs: Self) -> Self {
        *self * rhs
    }
}

impl<T: Scalar + ClosedAdd + ClosedMul + Total<Multiplicative>, const N: usize>
    Total<Multiplicative> for SquareMatrix<T, { N }>
{
}

impl<T: Scalar + ClosedAdd + ClosedMul + Associative<Multiplicative>, const N: usize>
    Associative<Multiplicative> for SquareMatrix<T, { N }>
{
}

impl<T: Scalar + ClosedAdd + ClosedMul + Commutative<Multiplicative>, const N: usize>
    Commutative<Multiplicative> for SquareMatrix<T, { N }>
{
}

impl<T: Scalar + ClosedAdd + ClosedMul + Identity<Multiplicative> + PartialEq, const N: usize>
    Identity<Multiplicative> for SquareMatrix<T, { N }>
{
    fn identity() -> Self {
        Self::broadcast(T::identity())
    }

    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}

// TODO: add matrix inverse
impl<T: Scalar + ClosedAdd + ClosedMul + Invertible<Multiplicative>, const N: usize>
    Invertible<Multiplicative> for SquareMatrix<T, { N }>
{
    fn inverse(&self) -> Self {
        unimplemented!()
    }

    fn inverted(&mut self) {
        *self = self.inverse()
    }
}

impl<T: Scalar + ComplexField + ClosedOps + Norm, const M: usize, const N: usize> Norm
    for Matrix<T, { M }, { N }>
{
    type Norm = T;
}

// TODO: The point of a Ring is to have Closed Addition and Closed Multiplication for Commutative Rings
// TODO: => Add this to Fructose
impl<T: Scalar + ClosedAdd + CommutativeRing, const M: usize, const N: usize> Module
    for Matrix<T, { M }, { N }>
{
    type Ring = T;
}

// TODO: A Vector Space shouldn't require a Real Field, a `Field` should be sufficient
impl<T: Scalar + ClosedOps + Field, const M: usize, const N: usize> VectorSpace
    for Matrix<T, { M }, { N }>
{
    type Field = T;
}

impl<
        T: Scalar + ClosedOps + ComplexField<RealField = T> + Norm + Sum,
        const M: usize,
        const N: usize,
    > NormedSpace for Matrix<T, { M }, { N }>
{
    type ComplexField = T;

    fn norm_squared(&self) -> Self::Norm {
        self.data
            .iter()
            .map(|col| col.iter().map(|e| *e * *e).sum())
            .sum()
    }

    fn norm(&self) -> Self::Norm {
        self.norm_squared().sqrt()
    }

    fn normalize(&mut self) {
        let magnitude = self.norm();
        self.data
            .iter_mut()
            .for_each(|col| col.iter_mut().for_each(|e| *e /= magnitude));
    }

    fn normalized(&self) -> Self {
        let mut mat = *self;
        mat.normalize();
        mat
    }
}

impl<
        T: Scalar + ClosedOps + ComplexField<RealField = T> + Norm + Sum + Zero + One,
        const N: usize,
    > InnerSpace for Vector<T, { N }>
{
    fn inner_product(&self, other: &Self) -> Self::ComplexField {
        self.dot(*other)
    }

    fn angle(&self, other: &Self) -> Self::Norm {
        let dot = self.dot(*other);
        let n1 = self.norm();
        let n2 = other.norm();

        if n1.is_zero() || n2.is_zero() {
            T::zero()
        } else {
            let prod = dot.real() / (n1 * n2);
            unimplemented!()
        }
    }
}

impl<T: Scalar + ClosedOps + ComplexField<RealField = T> + Norm, const N: usize> AffineSpace
    for Vector<T, { N }>
{
    type Translation = Self;

    fn translate_by(&self, translation: &Self::Translation) -> Self {
        *self + *translation
    }

    fn subtract(&self, rhs: &Self) -> Self::Translation {
        *self - *rhs
    }
}

// impl<T: Scalar + ClosedOps + ComplexField<RealField = T> + Norm, const N: usize> EuclideanSpace for Vector<T, { N }> {
//     type Coordinates = Point<T, {N}>;
//     type RealField = T;
//     const ORIGIN: Self = Self { data: [[0; N]; 1]};
//
//     fn scale_by(&self, scalar: Self::RealField) -> Self {
//         unimplemented!()
//     }
//
//     fn coordinates(&self) -> Self::Coordinates {
//         unimplemented!()
//     }
//
//     fn from_coordinates(coordinates: Self::Coordinates) -> Self {
//         unimplemented!()
//     }
//
//     fn distance_squared(&self, b: &Self) -> Self::RealField {
//         unimplemented!()
//     }
//
//     fn distance(&self, b: &Self) -> Self::RealField {
//         unimplemented!()
//     }
// }
