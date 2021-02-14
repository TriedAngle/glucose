pub trait MathComponent<T> {
    fn sqrt(&self) -> T;
    fn zero() -> T;
    fn one() -> T;
}

macro_rules! impl_vector_components {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    <$t>::sqrt(*self)
                }

                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1.0
                }
            }
        )*
    }
}

macro_rules! impl_vector_components_not_float {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    f32::sqrt(*self as f32) as $t
                }
                fn zero() -> Self {
                    <$t>::default()
                }

                fn one() -> Self {
                    1
                }
            }
        )*
    }
}

macro_rules! impl_vector_components_not_float_64 {
    ($($t:ty), *) => {
        $(
            impl MathComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    f64::sqrt(*self as f64) as $t
                }
                fn zero() -> Self {
                    <$t>::default()
                }
                fn one() -> Self {
                    1
                }
            }
        )*
    }
}

impl_vector_components!(f32, f64);
impl_vector_components_not_float!(i8, i16, i32);
impl_vector_components_not_float_64!(i64);