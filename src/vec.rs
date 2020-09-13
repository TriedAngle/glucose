#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use paste::paste;

#[macro_export]
macro_rules! vectors {
    ($($n:ident => [$t:ty; $d:expr]),+) => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $n {
                pub data: [$t; $d]
            }

            impl $n {
                #[inline]
                pub const fn new(data: [$t; $d]) -> Self {
                    Self {
                        data,
                    }
                }

                #[inline]
                pub fn unit_n(n: usize) -> Self {
                    let mut data = [<$t>::default(); $d];
                    data[n] = 1 as $t;
                    Self {
                        data,
                    }
                }

                #[inline]
                pub fn dot(&self, other: &$n) -> $t {
                    // TODO: this should scale O(n), maybe there is a better way?
                    let mut sum = self.data[0] * other.data[0];
                    for i in 1..$d {
                        sum += self.data[i] * other.data[i];
                    }
                    sum
                }

            }

            impl Add for $n {
                type Output = Self;
                #[inline]
                fn add(self, other: $n) -> Self {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = self.data[i] + other.data[i];
                    }
                    Self::new(data)
                }
            }

            impl AddAssign for $n {
                #[inline]
                fn add_assign(&mut self, other: $n) {
                    for i in 0..$d {
                        self.data[i] += other.data[i];
                    }
                }
            }

            impl Sub for $n {
                type Output = Self;
                #[inline]
                fn sub(self, other: $n) -> Self {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = self.data[i] - other.data[i];
                    }
                    Self::new(data)
                }
            }

            impl SubAssign for $n {
                #[inline]
                fn sub_assign(&mut self, other: $n) {
                    for i in 0..$d {
                        self.data[i] -= other.data[i];
                    }
                }
            }

            // Mul is maybe a little bit weird to have for vectors but it may come in handy?
            // maybe replace this with the dot or cross product?
            impl Mul for $n {
                type Output = Self;
                #[inline]
                fn mul(self, other: $n) -> Self {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = self.data[i] * other.data[i];
                    }
                    Self::new(data)
                }
            }

            /// vec' := vec1 *= vec2
            impl MulAssign for $n {
                #[inline]
                fn mul_assign(&mut self, other: $n) {
                    for i in 0..$d {
                        self.data[i] *= other.data[i];
                    }
                }
            }

            /// vec' = vec * scalar
            impl Mul<$t> for $n {
                type Output = Self;
                #[inline]
                fn mul(self, other: $t) -> Self {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = self.data[i] * other;
                    }
                    Self::new(data)
                }
            }

            /// vec' = scalar * vec
            impl Mul<$n> for $t {
                type Output = $n;
                #[inline]
                fn mul(self, other: $n) -> $n {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = other.data[i] * self;
                    }
                    $n::new(data)
                }
            }

            /// vec' := vec * scalar
            impl MulAssign<$t> for $n {
                #[inline]
                fn mul_assign(&mut self, other: $t) {
                    for i in 0..$d {
                        self.data[i] *= other;
                    }
                }
            }

            // Div is maybe a little bit weird to have for vectors but it may come in handy?
            impl Div for $n {
                type Output = Self;
                #[inline]
                fn div(self, other: $n) -> Self {
                    let mut data = [<$t>::default(); $d];
                    for i in 0..$d {
                         data[i] = self.data[i] / other.data[i];
                    }
                    Self::new(data)
                }
            }

            impl DivAssign for $n {
                #[inline]
                fn div_assign(&mut self, other: $n) {
                    for i in 0..$d {
                        self.data[i] /= other.data[i];
                    }
                }
            }

            impl DivAssign<$t> for $n {
                #[inline]
                fn div_assign(&mut self, other: $t) {
                    for i in 0..$d {
                        self.data[i] /= other;
                    }
                }
            }

            impl Neg for $n {
                type Output = $n;
                #[inline]
                fn neg(self) -> $n {
                    self * -1 as $t
                }
            }
        )+
    }
}

// This macro makes working with vectors easier by adding letters
#[macro_export]
macro_rules! letters_for_vectors {
    ($($n:ident => [$($c:expr => $d:ident),+]  {$t:ty}),+) => {
        $(
             impl $n {
                $(
                    #[inline]
                    pub const fn $d(&self) -> $t{
                        self.data[$c]
                    }

                    paste! {
                        #[inline]
                        pub fn [<set_ $d>](&mut self, value: $t) {
                            self.data[$c] = value;
                        }

                        #[inline]
                        pub fn [<unit_ $d>]() -> $n {
                            Self {
                                data: $n::unit_n($c).data
                            }
                        }
                    }
                )+
            }
        )+
    }
}

// define vectors and implement basics
// ($($n:ident => [$t:ty; $d:expr]),+)
vectors! {
    Vec1 => [f32; 1],
    Vec2 => [f32; 2],
    Vec3 => [f32; 3],
    Vec4 => [f32; 4],
    Vec5 => [f32; 5],
    Vec6 => [f32; 6],
    Vec7 => [f32; 7],
    Vec8 => [f32; 8]
}

// impl some letters for vectors
letters_for_vectors! {
    Vec1 => [0 => x] {f32},
    Vec2 => [0 => x, 1 => y] {f32},
    Vec3 => [0 => x, 1 => y, 2 => z] {f32},
    Vec4 => [0 => x, 1 => y, 2 => z, 3 => w] {f32}
}
