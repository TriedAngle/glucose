use std::ops::{Add, Mul, Div, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::traits::MathComponent;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, { N }> {
    fn default() -> Self {
        Self {
            data: [<T>::default(); N],
        }
    }
}

impl<T, const N: usize> Vector<T, { N }> {
    #[inline]
    pub const fn new(data: [T; N]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        N
    }
}

impl<T: Copy, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn broadcast(value: T) -> Self {
        Self {
            data: [value; N],
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Vector<T, { N }> {
    pub fn unit(n: usize) -> Self {
        let mut data = [<T>::default(); N];
        data[n] = <T>::one();
        Self {
            data
        }
    }

    #[inline]
    pub fn dot(&self, other: Self) -> T {
        let mut sum = <T>::default();
        for i in 0..N {
            sum += self.data[i] * other.data[i];
        }
        sum
    }

    #[inline]
    pub fn magnitude_squared(&self) -> T {
        let mut magnitude = <T>::default();
        for i in 0..N {
            magnitude += self.data[i] * self.data[i];
        }
        magnitude
    }

    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.data.iter_mut().for_each(|e| *e /= magnitude);
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut vec = *self;
        vec.normalize();
        vec
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.data.reverse()
    }

    #[inline]
    pub fn reversed(&self) -> Self {
        let mut vec = *self;
        vec.reverse();
        vec
    }

    #[inline]
    pub fn abs(&mut self) {
        self.data.iter_mut().for_each(|e| *e = e.abs());
    }

    // I can't find a good name for this, expect a rename in the future
    #[inline]
    pub fn abs_copy(&self) -> Self {
        let mut vec = *self;
        vec.abs();
        vec
    }

    #[inline]
    pub fn reflect(&mut self, normal: Self) {
        *self -= normal * <T>::from_i32(2) * self.dot(normal);
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        for i in 0..N {
            self.data[i] = self.data[i].maximum(min.data[i]).minimum(max.data[i])
        }
    }

    /// returns a VecN' with clamped values without consuming VecN
    #[inline]
    pub fn clamped(&self, min: Self, max: Self) -> Self {
        let mut vec = *self;
        vec.clamp(min, max);
        vec
    }

    #[inline]
    pub fn map<F: Fn(T) -> T>(&self, f: F) -> Self {
        let mut vector = *self;
        vector.data.iter_mut().for_each(|e| *e = f(*e));
        vector
    }

    #[inline]
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) {
        self.data.iter_mut().for_each(|e| *e = f(*e));
    }

    /// returns a new VecN' with each component having the bigger number from either VecN1 or VecN2
    #[inline]
    pub fn max_by_component(&self, other: &Self) -> Self {
        let mut vector = *self;
        for i in 0..N {
            vector.data[i] = self.data[i].maximum(other.data[i]);
        }
        vector
    }

    /// returns a new VecN' with each component having the smaller number from either VecN1 or VecN2
    #[inline]
    pub fn min_by_component(&self, other: &Self) -> Self {
        let mut vector = *self;
        for i in 0..N {
            vector.data[i] = self.data[i].minimum(other.data[i]);
        }
        vector
    }

    /// constructs a new VecN with all values being 0
    #[inline]
    pub fn zero() -> Self {
        Self::broadcast(<T>::zero())
    }

    /// constructs a new VecN with all values being 1
    #[inline]
    pub fn one() -> Self {
        Self::broadcast(<T>::one())
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, { N }> {
    fn from(arr: [T; N]) -> Self {
        Self { data: arr }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Add for Vector<T, { N }> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> AddAssign for Vector<T, { N }> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Sub for Vector<T, { N }> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SubAssign for Vector<T, { N }> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Mul for Vector<T, { N }> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] * rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> MulAssign for Vector<T, { N }> {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] *= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Div for Vector<T, { N }> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] / rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> DivAssign for Vector<T, { N }> {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] /= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Mul<T> for Vector<T, { N }> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] * rhs;
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> MulAssign<T> for Vector<T, { N }> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Div<T> for Vector<T, { N }> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] / rhs;
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> DivAssign<T> for Vector<T, { N }> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] /= rhs;
        }
    }
}

// impl<T: MathComponent<T>, const N: usize> Mul<Vector<T, { N }>> for T {
//     type Output = Vector<T, { N }>;
//
//     fn mul(self, rhs: Vector<T, { N }>) -> Self::Output {
//         let mut data = [<T>::default(); N];
//         data.iter_mut().for_each(|e| *e *= self);
//         Self { data }
//     }
// }

macro_rules! vec_short {
    ($($n:ident => $t:ty),+) => {
        $(
            type $n<const N: usize> = Vector<$t, N>;
        )+
    };
    ($($n:ident => $d:expr),+) => {
        $(
            type $n<T> = Vector<T, $d>;
        )+
    };
    ($($n:ident => [$t:ty; $d:expr]),+) => {
        $(
            type $n = Vector<$t, $d>;
        )+
    }
}


#[test]
fn test() {
    let vec = Vector::new([1.0, 3.0, 3.0]);
    let vec2 = vec.normalized();
    println!("{:?}", vec);
    println!("{:?}", vec2);
}
// A macro to add lettered getter and setter functions to a VecN
//
// # Usage
// * `<...>` : replace with values
// * `,+` can repeat infinite times but can not be empty
//
// `<Vector Name> => [<<index for data[n]> => <corresponding letter>>,+] <{type of Vector: T}>,+`
//
// # Example
// ```
// letters_for_vectors! {
//     Vec2 => [0 => x, 1 => y] {f32},
//     Vec3 => [0 => x, 1 => y, 2 => z] {f32}
// }
// ```
// #[macro_export]
// macro_rules! letters_for_vectors {
//     ($($n:ident => [$($c:expr => $d:ident),+]  {$t:ty}),+) => {
//         $(
//              impl $n {
//                 $(
//                     /// Returns `data[n]` from the corresponding letter
//                     #[inline]
//                     pub const fn $d(&self) -> $t{
//                         self.data[$c]
//                     }
//
//                     paste! {
//                         /// sets the `data[n]` to the value f
//                         #[inline]
//                         pub fn [<set_ $d>](&mut self, value: $t) {
//                             self.data[$c] = value;
//                         }
//
//                         // Creates a new unit vector from the corresponding letter
//                         #[inline]
//                         pub fn [<unit_ $d>]() -> $n {
//                             Self {
//                                 data: $n::unit_n($c).data
//                             }
//                         }
//                     }
//                 )+
//             }
//         )+
//     }
// }

// A macro to get Vec(n-1) from Vec(n) by using lettered functions
//
// # Usage
// * `<...>` : replace with values
// * `,+` can repeat infinite times
//
// `<VectorN Name> => <VectorN-1> from <{letter(s)} => [<indices for data[n]>,+]}>,+>,+`
//
// # Example
// ```
// vectors_from_letters! {
//     Vec3 => Vec2 from {xy => [0, 1], xz => [0, 2], zy => [1, 2]},
//     Vec4 => Vec3 from {xyz => [0, 1, 2], xyw => [0, 1, 3], xzw => [0, 2, 3], yzw => [1, 2, 3]}
// }
// ```
// #[macro_export]
// macro_rules! vectors_from_letters {
//     ($($n:ident => $m:ident from {$($v:ident => $d:expr),+}),+) => {
//         $(
//              impl $n {
//                 $(
//                     pub fn $v(&self) -> $m {
//                         // this is a 'hack' to get the same type
//                         let mut vec = $m::broadcast(self.data[0]);
//
//                         for (i, e) in $d.iter().enumerate() {
//                             vec.data[i] = self.data[*e];
//                         }
//                         vec
//                     }
//                 )+
//             }
//         )+
//     }
// }

// A macro to convert between Vectors
//
// # Usage
// * `<...>` : replace with values
// * `,+` / `;+` can repeat infinite times but can not be empty with
// * `,*` / `;*` can repeat infinite times and can be empty with
//
// * Before the first `=>`: all vectors bigger than the target vector
// * Between the first and the secodn `=>`: target vector
// * After the second `=>`: all vectors smaller than the target vector
//
// `[<<Vector Name>, <Dimension>>;*] => [<<Vector Name>, <Dimension>>] {type} => [<<Vector Name>, <Dimension>>;*],+`
//
// # Example
// ```
// vectors_from_letters! {
// [Vec4, 4; Vec3, 3] => [Vec2, 2] {f32} => [Vec1, 1],
// [Vec5, 5; Vec4, 4] => [Vec3, 3] {f32} => [Vec2, 2; Vec1, 1],
// [] => [Vec4, 4] {f32} => [Vec3, 3; Vec2, 2; Vec1, 1],
// }
// ```
// #[macro_export]
// macro_rules! vector_to_vector {
//     ($([$($ob:ident, $obd:expr);*] => [$n:ident, $d:expr] {$t:ty} => [$($os:ident, $osd:expr);*]),+) => {
//         $(
//             $(
//                 impl From<$ob> for $n {
//                     #[inline]
//                     fn from(vec: $ob) -> $n {
//                         let mut data = [1 as $t; $d];
//                         for i in 0..$d {
//                             data[i] = vec.data[i];
//                         }
//                         Self::new(data)
//                     }
//                 }
//             )*
//
//             $(
//                 impl From<$os> for $n {
//                     #[inline]
//                     fn from(vec: $os) -> $n {
//                         let mut data = [1 as $t; $d];
//                         for i in 0..$osd {
//                             data[i] = vec.data[i];
//                         }
//                         Self::new(data)
//                     }
//                 }
//             )*
//         )+
//     }
// }

// this is a temporary macro to implement the cross product only for the Vec3
// #[macro_export]
// macro_rules! cross_product {
//     ($n:ident) => {
//         impl $n {
//             #[inline]
//             pub fn cross(&self, other: &$n) -> $n {
//                 let mut data = self.data.clone();
//                 data[0] = self.x().mul_add(other.z(), -self.z() * other.y());
//                 data[1] = self.y().mul_add(other.x(), -self.x() * other.z());
//                 data[2] = self.z().mul_add(other.y(), -self.y() * other.x());
//                 $n::new(data)
//             }
//         }
//     }
// }
//
// vectors! {
//     Vec1 => [f32; 1],
//     Vec2 => [f32; 2],
//     Vec3 => [f32; 3],
//     Vec4 => [f32; 4],
//     Vec5 => [f32; 5],
//     Vec6 => [f32; 6],
//     Vec7 => [f32; 7],
//     Vec8 => [f32; 8]
// }
//
// vector_to_vector!{
//     [Vec8, 8; Vec7, 7; Vec6, 6; Vec5, 5; Vec4, 4; Vec3, 3; Vec2, 2] => [Vec1, 1] {f32} => [],
//     [Vec8, 8; Vec7, 7; Vec6, 6; Vec5, 5; Vec4, 4; Vec3, 3] => [Vec2, 2] {f32} => [Vec1, 1],
//     [Vec8, 8; Vec7, 7; Vec6, 6; Vec5, 5; Vec4, 4] => [Vec3, 3] {f32} => [Vec2, 2; Vec1, 1],
//     [Vec8, 8; Vec7, 7; Vec6, 6; Vec5, 5] => [Vec4, 4] {f32} => [Vec3, 3; Vec2, 2; Vec1, 1],
//     [Vec8, 8; Vec7, 7; Vec6, 6] => [Vec5, 5] {f32} => [Vec4, 4; Vec3, 3; Vec2, 2; Vec1, 1],
//     [Vec8, 8; Vec7, 7] => [Vec6, 6] {f32} => [Vec5, 5; Vec4, 4; Vec3, 3; Vec2, 2; Vec1, 1],
//     [Vec8, 8] => [Vec7, 7] {f32} => [Vec6, 6; Vec5, 5; Vec4, 4; Vec3, 3; Vec2, 2; Vec1, 1],
//     [] => [Vec8, 8] {f32} => [Vec7, 7; Vec6, 6; Vec5, 5; Vec4, 4; Vec3, 3; Vec2, 2; Vec1, 1]
// }
//
//
// letters_for_vectors! {
//     Vec1 => [0 => x] {f32},
//     Vec2 => [0 => x, 1 => y] {f32},
//     Vec3 => [0 => x, 1 => y, 2 => z] {f32},
//     Vec4 => [0 => x, 1 => y, 2 => z, 3 => w] {f32}
// }
//
// vectors_from_letters! {
//     Vec2 => Vec1 from {xx => [0], yy => [1]},
//     Vec3 => Vec2 from {xy => [0, 1], xz => [0, 2], zy => [1, 2]},
//     Vec4 => Vec3 from {xyz => [0, 1, 2], xyw => [0, 1, 3], xzw => [0, 2, 3], yzw => [1, 2, 3]}
// }
//
// cross_product!(Vec3);
