#[macro_export]
macro_rules! vectors {
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

vectors! {
  Vec2 => [f32; 2],
  Vec3 => [f32; 3],
  Vec4 => [f32; 4]
}