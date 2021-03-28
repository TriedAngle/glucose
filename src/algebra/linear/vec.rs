use crate::algebra::linear::mat::Matrix;
use crate::algebra::linear::scalar::Scalar;
use fructose::algebra::helpers::identity::{Two, One};
use fructose::operators::{ClosedAdd, ClosedMul, ClosedSub};
use std::ops::{Index, IndexMut};

pub type Point<T, const N: usize> = Vector<T, { N }>;

pub type Vector<T, const N: usize> = Matrix<T, { N }, 1>;

pub type RowVector<T, const N: usize> = Matrix<T, 1, { N }>;

impl<T: Scalar + Copy + ClosedMul + ClosedAdd, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn dot(&self, other: Self) -> T {
        let mut sum = <T>::default();
        for i in 0..N {
            sum += self[i] * other[i];
        }
        sum
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.data[0].reverse()
    }

    #[inline]
    pub fn reversed(&self) -> Self {
        let mut vec = *self;
        vec.reverse();
        vec
    }
}

impl<T: Scalar + One, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn unit(n: usize) -> Self {
        let mut data = [<T>::default(); N];
        data[n] = <T>::one();
        Self { data: [data] }
    }
}

impl<T: Scalar + Two + ClosedMul + ClosedAdd + ClosedSub, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn reflect(&mut self, normal: Self) {
        *self -= normal * <T>::two() * self.dot(normal);
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, { N }> {
    fn from(rhs: [T; N]) -> Self {
        Point::new([rhs])
    }
}

impl<T> From<(T, T)> for Point<T, 2> {
    fn from(rhs: (T, T)) -> Self {
        Point::new([[rhs.0, rhs.1]])
    }
}

impl<T> From<(T, T, T)> for Point<T, 3> {
    fn from(rhs: (T, T, T)) -> Self {
        Point::new([[rhs.0, rhs.1, rhs.2]])
    }
}

impl<T> From<(T, T, T, T)> for Point<T, 4> {
    fn from(rhs: (T, T, T, T)) -> Self {
        Point::new([[rhs.0, rhs.1, rhs.2, rhs.3]])
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, { N }> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[0][index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, { N }> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[0][index]
    }
}

// TODO: reimplement those?
// #[macro_export]
// macro_rules! vec_short {
//     ($($n:ident => $t:ty),+) => {
//         $(
//             type $n<const N: usize> = Vector<$t, N>;
//         )+
//     };
//     ($($n:ident => $d:expr),+) => {
//         $(
//             type $n<T> = Vector<T, $d>;
//         )+
//     };
//     ($($n:ident => [$t:ty; $d:expr]),+) => {
//         $(
//             type $n = Vector<$t, $d>;
//         )+
//     }
// }
//
// /// A macro to add lettered getter and setter functions to a VecN
// ///
// /// # Usage
// /// * `<...>` : replace with values
// /// * `,+` can repeat infinite times but can not be empty
// ///
// /// `<Vector Dimension> => [<<index for data[n]> => <corresponding letter>>,+],+`
// ///
// /// # Example
// /// ```
// // letters_for_vectors! {
// //     2 => [0, x; 1, y],
// //     3 => [0, x; 1, y; 2, z]
// // }
// /// ```
// // TODO: add recursive parsing so the inner expression can be omitted
// #[macro_export]
// macro_rules! letters_for_vectors {
//     ($($e:expr => [$($c:expr, $d:ident);+]),+) => {
//         $(
//             impl<T: Scalar> Vector<T, $e> {
//                 $(
//                     #[inline]
//                     pub const fn $d(&self) -> T {
//                         self.data[$c]
//                     }
//
//                     paste! {
//                         /// sets the `data[n]` to the value f
//                         #[inline]
//                         pub fn [<set_ $d>](&mut self, value: T) {
//                             self.data[$c] = value;
//                         }
//
//                         // Creates a new unit vector from the corresponding letter
//                         #[inline]
//                         pub fn [<unit_ $d>]() -> Self {
//                             Self::unit($c)
//                         }
//                     }
//
//                 )+
//             }
//         )+
//     }
// }
