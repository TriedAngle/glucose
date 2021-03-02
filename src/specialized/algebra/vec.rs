use crate::algebra::linear::scalar::Scalar;
use crate::algebra::linear::vec::Vector;
use crate::forward;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

macro_rules! specialize_vec {
    ($($name:ident => $dim:expr => $inner:ident [$($index:expr),*]),*) => {
        $(
            #[derive(Copy, Clone, PartialEq)]
            pub struct $name<T> {
                inner: $inner<T, $dim>,
            }

            impl<T> $name<T> {
                pub fn new(data: [T; $dim]) -> Self {
                    Self { inner: $inner::new(data) }
                }

                pub fn as_inner(self) -> $inner<T, $dim> {
                    self.inner
                }

                pub fn as_inner_ref(&self) -> &$inner<T, $dim> {
                    &self.inner
                }

                pub fn as_inner_ref_mut(&mut self) -> &mut $inner<T, $dim> {
                    &mut self.inner
                }


                forward! {
                    to_const_inner
                    $inner => fn len(&self) -> usize;
                    $inner => fn as_array(&self) -> &[T; $dim];
                    $inner => fn as_ptr(&self) -> *const T;
                }

                forward! {
                    to_inner
                    $inner => fn as_slice(&self) -> &[T];
                    $inner => fn as_bytes(&self) -> &[u8];
                }
                forward! {
                    to_inner_mut
                    $inner => fn as_array_mut(&mut self) -> &mut [T; $dim];
                    $inner => fn as_ptr_mut(&mut self) -> *mut T;
                    $inner => fn as_slice_mut(&mut self) -> &mut [T];
                }
            }

            impl<T: Copy> $name<T> {
                #[inline]
                pub fn broadcast(value: T) -> Self {
                    Self {
                        inner: $inner { data: [value; $dim] }
                    }
                }
            }

            impl<T: Scalar> $name<T> {
                #[inline]
                pub fn unit(n: usize) -> Self {
                    let mut data = [T::default(); $dim];
                    data[n] = T::one();
                    Self::new(data)
                }

                pub fn reversed(&self) -> Self {
                    let mut vec = *self;
                    vec.reverse();
                    vec
                }

                #[inline]
                pub fn map<F: Fn(T) -> T>(&self, f: F) -> Self {
                    let mut vec = *self;
                    vec.inner.data.iter_mut().for_each(|e| *e = f(*e));
                    vec
                }

                #[inline]
                pub fn apply<F: Fn(T) -> T>(&mut self, f: F) {
                    self.inner.data.iter_mut().for_each(|e| *e = f(*e));
                }

                forward! {
                    to_inner_mut
                    $inner => fn reverse(&mut self) -> ();
                }

                forward! {
                    to_inner_type
                    $inner => fn zero() -> $name;
                    $inner => fn one() -> $name;
                }
            }

            impl<T> From<[T; $dim]> for $name<T> {
                #[inline]
                fn from(arr: [T; $dim]) -> Self {
                    Self { inner: Vector::from(arr) }
                }
            }

            impl<T> From<$name<T>> for $inner<T, $dim> {
                #[inline]
                fn from(rhs: $name<T>) -> Self {
                    rhs.inner
                }
            }

            impl<T: Debug> Debug for $name<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    f.write_str(&format!("Vec{} {{ {:?} }}\n", $dim, self.inner.data))
                }
            }

            impl<T: Default + Copy> Default for $name<T> {
                #[inline]
                fn default() -> Self {
                    Self { inner: $inner::default() }
                }
            }

            impl<T: Scalar> Add for $name<T> {
                type Output = Self;
                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self {
                        inner: $inner {
                            data: [
                                $(self.inner.data[$index] + rhs.inner.data[$index],)*
                            ]
                        }
                    }
                }
            }

            impl<T: Scalar> Sub for $name<T> {
                type Output = Self;
                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    Self {
                        inner: $inner {
                            data: [
                                $(self.inner.data[$index] - rhs.inner.data[$index],)*
                            ]
                        }
                    }
                }
            }

            impl<T: Scalar> Mul<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn mul(self, rhs: T) -> Self::Output {
                    Self {
                        inner: $inner {
                            data: [
                                $(self.inner.data[$index] * rhs,)*
                            ]
                        }
                    }
                }
            }

            impl<T: Scalar> Div<T> for $name<T> {
                type Output = Self;
                #[inline]
                fn div(self, rhs: T) -> Self::Output {
                    Self {
                        inner: $inner {
                            data: [
                                $(self.inner.data[$index] / rhs,)*
                            ]
                        }
                    }
                }
            }
            impl<T: Scalar> AddAssign for $name<T> {
                #[inline]
                fn add_assign(&mut self, rhs: $name<T>) {
                    $(self.inner.data[$index] += rhs.inner.data[$index];)*
                }
            }

            impl<T: Scalar> SubAssign for $name<T> {
                #[inline]
                fn sub_assign(&mut self, rhs: $name<T>) {
                    $(self.inner.data[$index] *= rhs.inner.data[$index];)*
                }
            }

            impl<T: Scalar> MulAssign<T> for $name<T> {
                #[inline]
                fn mul_assign(&mut self, rhs: T) {
                    $(self.inner.data[$index] *= rhs;)*
                }
            }

            impl<T> Index<usize> for $name<T> {
                type Output = T;

                fn index(&self, index: usize) -> &Self::Output {
                    &self.inner[index]
                }
            }

            impl<T> IndexMut<usize> for $name<T> {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    &mut self.inner[index]
                }
            }
        )*
    }
}

specialize_vec! {
    Vec2 => 2 => Vector [0, 1],
    Vec3 => 3 => Vector [0, 1, 2],
    Vec4 => 4 => Vector [0, 1, 2, 3],
    Vec5 => 5 => Vector [0, 1, 2, 3, 4],
    Vec6 => 6 => Vector [0, 1, 2, 3, 4, 5],
    Vec7 => 7 => Vector [0, 1, 2, 3, 4, 5, 6],
    Vec8 => 8 => Vector [0, 1, 2, 3, 4, 5, 6, 7]
}

#[cfg(test)]
mod specialized_vec_tests {
    use super::*;
    #[test]
    fn addition() {
        let vec1 = Vec3::new([2.0, 3.0, -1.0]);
        let vec2 = Vec3::new([-1.0, 2.0, 4.0]);
        let vec3 = vec1 + vec2;
    }

    #[test]
    #[ignore]
    fn speed_comparison() {
        // the VecN "specializations" implement their methods without for loops.
        // maybe that is why they are >1000x times faster?
        use std::time::Instant;
        let vec_n_adder: Vector<f64, 8> = Vector::new([2.0, 3.0, -2.0, 1.0, 2.0, 3.0, -1.0, 1.0]);
        let vec_4_adder: Vec8<f64> = Vec8::new([2.0, 3.0, -2.0, 1.0, 2.0, 3.0, -1.0, 1.0]);

        let mut vec_n: Vector<f64, 8> = Vector::zero();
        let mut vec_4: Vec8<f64> = Vec8::zero();

        let start_n = Instant::now();
        for _ in 0..10000000 {
            vec_n = vec_n + vec_n_adder;
        }
        let stop_n = start_n.elapsed();

        let start_4 = Instant::now();
        for _ in 0..10000000 {
            vec_4 = vec_4 + vec_4_adder;
        }
        let stop_4 = start_4.elapsed();

        println!("Vector<f64, 4> took: {}", stop_n.as_secs_f32());
        println!("Vec4<f64> took: {}", stop_4.as_secs_f32());
    }
}
