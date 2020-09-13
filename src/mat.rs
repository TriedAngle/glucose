use crate::vec::{Vec2, Vec3, Vec4, Vec5, Vec6, Vec7, Vec8};

#[macro_export]
macro_rules! matrices {
    ($($n:ident => [$t:ty; $d:expr]),+) => {
        $(
            #[derive(Clone, Copy, Debug, Default)]
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
            }
        )+
    }
}

matrices! {
  Mat2 => [Vec2; 2],
  Mat3 => [Vec3; 3],
  Mat4 => [Vec4; 4],
  Mat5 => [Vec5; 5],
  Mat6 => [Vec6; 6],
  Mat7 => [Vec7; 7],
  Mat8 => [Vec8; 8]
}
