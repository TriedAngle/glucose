#![allow(dead_code)]

// use crate::vec::{Vec1, Vec2, Vec3, Vec4, Vec5, Vec6, Vec7, Vec8};

// #[macro_export]
// macro_rules! matrices {
//     ($($n:ident {$t:ty}=> [$tv:ty; $d:expr]),+) => {
//         $(
//             #[derive(Clone, Copy, Debug, Default)]
//             #[repr(C)]
//             pub struct $n {
//                 pub data: [$tv; $d]
//             }
//
//             impl $n {
//                 #[inline]
//                 pub const fn new(data: [$tv; $d]) -> Self {
//                     Self {
//                         data,
//                     }
//                 }
//
//                 #[inline]
//                 pub fn new_identity() -> Self {
//                     let mut matrix = Self::zero();
//                     for i in 0..$d {
//                         matrix.data[i].data[i] = 1 as $t
//                     }
//                     matrix
//                 }
//
//                 pub fn determinant(&self) -> $t {
//
//                 }
//
//                 pub fn zero() -> Self {
//                     Self::new([<$tv>::zero(); $d])
//                 }
//             }
//         )+
//     }
// }

// matrices! {
//     Mat1 {f32} => [Vec1; 1],
//     Mat2 {f32} => [Vec2; 2],
//     Mat3 {f32} => [Vec3; 3],
//     Mat4 {f32} => [Vec4; 4],
//     Mat5 {f32} => [Vec5; 5],
//     Mat6 {f32} => [Vec6; 6],
//     Mat7 {f32} => [Vec7; 7],
//     Mat8 {f32} => [Vec8; 8]
// }
