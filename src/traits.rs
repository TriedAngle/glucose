pub trait VectorComponent<T> {
    fn sqrt(&self) -> T;
}

macro_rules! impl_vector_components {
    ($($t:ty), *) => {
        $(
            impl VectorComponent<$t> for $t {
                fn sqrt(&self) -> Self {
                    <$t>::sqrt(*self)
                }
            }
        )*
    }
}

impl_vector_components!(f32, f64);