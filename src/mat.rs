use crate::vec::{Vec2, Vec3, Vec4};

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
  Mat4 => [Vec4; 4]
}