#[macro_export]
macro_rules! forward {
    ($(fn $method:ident(self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            fn $method(self $(, $arg: $ty )*) -> $rty {
                Self::$method(self $(, $arg)*)
            }
        )*
    };
    (to_inner $($inner:ident => fn $method:ident(&self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            pub fn $method(&self $(, $arg: $ty )*) -> $rty {
                self.inner.$method($(, $arg)*)
            }
        )*
    };
    (to_inner_type $($inner:ident => fn $method:ident() -> $outer:ident;)*) => {
        $(
            #[inline]
            pub fn $method() -> $outer<T> {
                $outer::new($inner::$method().data)
            }
        )*
    };
    (to_inner_mut $($inner:ident => fn $method:ident(&mut self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            pub fn $method(&mut self $(, $arg: $ty )*) -> $rty {
                self.inner.$method($(, $arg)*)
            }
        )*
    };
    (to_const_inner $($inner:ident => fn $method:ident(&self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            pub const fn $method(&self $(, $arg: $ty )*) -> $rty {
                self.inner.$method($(, $arg)*)
            }
        )*
    }
}
