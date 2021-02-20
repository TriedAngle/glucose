macro_rules! forward {
    ($(Self::$method:ident(self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            fn $method(self $(, $arg: $ty )*) -> $rty {
                Self::$method(self $(, $arg)*)
            }
        )*
    }
}
