pub trait Scalar: PartialEq + Copy + Default {}

impl<T: PartialEq + Copy + Default> Scalar for T {}
