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

                pub const fn broadcast(value: $t) -> Self {
                    let data = [value; $d];
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
                pub fn dot(&self, other: Self) -> $t {
                    let mut sum = self.data[0] * other.data[0];
                    for i in 1..$d {
                        sum += self.data[i] * other.data[i];
                    }
                    sum
                }

                #[inline]
                pub fn magnitude_squared(&self) -> $t {
                    let mut magnitude = <$t>::default();
                    for i in 0..$d {
                        magnitude += self.data[i] * self.data[i];
                    }
                    magnitude
                }

                #[inline]
                pub fn magnitude(&self) -> $t {
                    self.magnitude_squared().sqrt()
                }

                #[inline]
                pub fn normalize(&mut self) {
                    let magnitude = self.magnitude();
                    for i in 0..$d {
                        self.data[i] /= magnitude;
                    }
                }

                #[inline]
                pub fn normalized(&self) -> Self {
                    let mut vector = self.clone();
                    vector.normalize();
                    vector
                }

                #[inline]
                pub fn reverse(&mut self) {
                    self.data.reverse();
                }

                #[inline]
                pub fn reversed(&self) -> Self {
                    let mut vector = self.clone();
                    vector.reverse();
                    vector
                }

                #[inline]
                pub fn abs(&mut self) {
                    for i in 0..$d {
                        self.data[i] = self.data[i].abs();
                    }
                }

                #[inline]
                pub fn absd(&self) -> Self {
                    let mut vector = self.clone();
                    vector.abs();
                    vector
                }

                #[inline]
                pub fn reflect(&mut self, normal: Self) {
                    *self -= 2 as $t * self.dot(normal) * normal;
                }

                #[inline]
                pub fn reflected(&self, normal: $n) -> Self {
                    let mut vector = *self;
                    vector.reflect(normal);
                    vector
                }

                #[inline]
                pub fn clamp(&mut self, min: Self, max: Self) {
                    for i in 0..$d {
                        self.data[i] = self.data[i].max(min.data[i]).min(max.data[i])
                    }
                }

                #[inline]
                pub fn clamped(&self, min: Self, max: Self) -> Self {
                    let mut vector = self.clone();
                    vector.clamp(min, max);
                    vector
                }

               #[inline]
                pub fn map<F: Fn($t) -> $t>(&self, f: F) -> Self {
                    let mut vector = self.clone();
                    for i in 0..$d {
                        vector.data[i] = f(vector.data[i]);
                    }
                    vector
                }

                #[inline]
                pub fn apply<F: Fn($t) -> $t>(&mut self, f: F) {
                    for i in 0..$d {
                        self.data[i] = f(self.data[i]);
                    }
                }

                #[inline]
                pub fn max_by_component(mut self, other: Self) -> Self {
                   for i in 0..$d {
                        self.data[i] = self.data[i].max(other.data[i]);
                   }
                   self
                }

                #[inline]
                pub fn min_by_component(mut self, other: Self) -> Self {
                   for i in 0..$d {
                        self.data[i] = self.data[i].min(other.data[i]);
                   }
                   self
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::broadcast(0 as $t)
                }

                #[inline]
                pub fn one() -> Self {
                    Self::broadcast(1 as $t)
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
                        // Create a new unit vector $d
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

#[macro_export]
macro_rules! vectors_from_letters {
    ($($n:ident => $m:ident from {$($v:ident => $d:expr),+}),+) => {
        $(
             impl $n {
                $(
                    pub fn $v(&self) -> $m {
                        let data = self.data.clone();
                        let mut vec = $m::broadcast(data[0]);
                        // using a counter looks weird... maybe there is a better way?
                        let mut counter = 0;
                        for i in &$d {
                            vec.data[counter] = data[*i];
                            counter += 1;
                        }
                        vec
                    }
                )+
            }
        )+
    }
}

// this is a temporary macro to implement the cross product only for the Vec3
#[macro_export]
macro_rules! cross_product {
    ($n:ident) => {
        impl $n {
            #[inline]
            pub fn cross(&self, other: &$n) -> $n {
                let mut data = self.data.clone();
                data[0] = self.x().mul_add(other.z(), -self.z() * other.y());
                data[1] = self.y().mul_add(other.x(), -self.x() * other.z());
                data[2] = self.z().mul_add(other.y(), -self.y() * other.x());
                $n::new(data)
            }
        }
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

vectors_from_letters! {
    Vec3 => Vec2 from {xy => [0, 1], xz => [0, 2], zy => [1, 2]},
    Vec4 => Vec3 from {xyz => [0, 1, 2], xyw => [0, 1, 3], xzw => [0, 2, 3], yzw => [1, 2, 3]}
}

cross_product!(Vec3);
