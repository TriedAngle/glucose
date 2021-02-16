use crate::mat::Matrix;
use crate::traits::MathComponent;
use paste::paste;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, { N }> {
    #[inline]
    fn default() -> Self {
        Self {
            data: [<T>::default(); N],
        }
    }
}

impl<T: Display, const N: usize> Display for Vector<T, { N }> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for n in 0..N {
            &string.push_str(&format!("|{}|\n", self.data[n]));
        }
        write!(f, "{}", string)
    }
}

impl<T, const N: usize> Vector<T, { N }> {
    #[inline]
    pub const fn new(data: [T; N]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        N
    }
}

impl<T: Copy, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn broadcast(value: T) -> Self {
        Self { data: [value; N] }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn unit(n: usize) -> Self {
        let mut data = [<T>::default(); N];
        data[n] = <T>::one();
        Self { data }
    }

    #[inline]
    pub fn dot(&self, other: Self) -> T {
        let mut sum = <T>::default();
        for i in 0..N {
            sum += self.data[i] * other.data[i];
        }
        sum
    }

    #[inline]
    pub fn magnitude_squared(&self) -> T {
        let mut magnitude = <T>::default();
        for i in 0..N {
            magnitude += self.data[i] * self.data[i];
        }
        magnitude
    }

    #[inline]
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.data.iter_mut().for_each(|e| *e /= magnitude);
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut vec = *self;
        vec.normalize();
        vec
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.data.reverse()
    }

    #[inline]
    pub fn reversed(&self) -> Self {
        let mut vec = *self;
        vec.reverse();
        vec
    }

    #[inline]
    pub fn abs(&mut self) {
        self.data.iter_mut().for_each(|e| *e = e.abs());
    }

    // I can't find a good name for this, expect a rename in the future
    #[inline]
    pub fn abs_copy(&self) -> Self {
        let mut vec = *self;
        vec.abs();
        vec
    }

    #[inline]
    pub fn reflect(&mut self, normal: Self) {
        *self -= normal * <T>::two() * self.dot(normal);
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        for i in 0..N {
            self.data[i] = self.data[i].maximum(min.data[i]).minimum(max.data[i])
        }
    }

    /// returns a VecN' with clamped values without consuming VecN
    #[inline]
    pub fn clamped(&self, min: Self, max: Self) -> Self {
        let mut vec = *self;
        vec.clamp(min, max);
        vec
    }

    #[inline]
    pub fn map<F: Fn(T) -> T>(&self, f: F) -> Self {
        let mut vector = *self;
        vector.data.iter_mut().for_each(|e| *e = f(*e));
        vector
    }

    #[inline]
    pub fn apply<F: Fn(T) -> T>(&mut self, f: F) {
        self.data.iter_mut().for_each(|e| *e = f(*e));
    }

    /// returns a new VecN' with each component having the bigger number from either VecN1 or VecN2
    #[inline]
    pub fn max_by_component(&self, other: &Self) -> Self {
        let mut vector = *self;
        for i in 0..N {
            vector.data[i] = self.data[i].maximum(other.data[i]);
        }
        vector
    }

    /// returns a new VecN' with each component having the smaller number from either VecN1 or VecN2
    #[inline]
    pub fn min_by_component(&self, other: &Self) -> Self {
        let mut vector = *self;
        for i in 0..N {
            vector.data[i] = self.data[i].minimum(other.data[i]);
        }
        vector
    }

    /// constructs a new VecN with all values being 0
    #[inline]
    pub fn zero() -> Self {
        Self::broadcast(<T>::zero())
    }

    /// constructs a new VecN with all values being 1
    #[inline]
    pub fn one() -> Self {
        Self::broadcast(<T>::one())
    }
}

// this seems a bit slow, maybe find a better way?
impl<T: Default + Copy, const N: usize> Vector<T, { N }> {
    #[inline]
    pub fn from<const M: usize>(rhs: Vector<T, { M }>) -> Self {
        if N == M {
            // I have no idea how to cast Array of size M to size N
            // this works for now I guess?
            let pointer = &rhs.data as *const T;
            let data: [T; N] = unsafe { *pointer.cast() };
            Self::new(data)
        } else if N > M {
            let mut data = [<T>::default(); N];
            for i in 0..M {
                data[i] = rhs.data[i];
            }
            Self::new(data)
        } else {
            let mut data = [<T>::default(); N];
            for i in 0..N {
                data[i] = rhs.data[i];
            }
            Self::new(data)
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, { N }> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self { data: arr }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Add for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> AddAssign for Vector<T, { N }> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Sub for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> SubAssign for Vector<T, { N }> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Mul for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] * rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> MulAssign for Vector<T, { N }> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] *= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Div for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] / rhs.data[i];
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> DivAssign for Vector<T, { N }> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] /= rhs.data[i];
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Mul<T> for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] * rhs;
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> MulAssign<T> for Vector<T, { N }> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> Div<T> for Vector<T, { N }> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let mut data = [<T>::default(); N];
        for i in 0..N {
            data[i] = self.data[i] / rhs;
        }
        Self { data }
    }
}

impl<T: MathComponent<T> + Copy, const N: usize> DivAssign<T> for Vector<T, { N }> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.data[i] /= rhs;
        }
    }
}

impl<T: Copy, const N: usize> From<Matrix<T, { N }, 1>> for Vector<T, { N }> {
    #[inline]
    fn from(rhs: Matrix<T, { N }, 1>) -> Self {
        Self { data: rhs.data[0] }
    }
}

#[macro_export]
macro_rules! vec_short {
    ($($n:ident => $t:ty),+) => {
        $(
            type $n<const N: usize> = Vector<$t, N>;
        )+
    };
    ($($n:ident => $d:expr),+) => {
        $(
            type $n<T> = Vector<T, $d>;
        )+
    };
    ($($n:ident => [$t:ty; $d:expr]),+) => {
        $(
            type $n = Vector<$t, $d>;
        )+
    }
}

/// A macro to add lettered getter and setter functions to a VecN
///
/// # Usage
/// * `<...>` : replace with values
/// * `,+` can repeat infinite times but can not be empty
///
/// `<Vector Dimension> => [<<index for data[n]> => <corresponding letter>>,+],+`
///
/// # Example
/// ```
/// letters_for_vectors! {
///     2 => [0, x; 1, y],
///     3 => [0, x; 1, y; 2, z]
/// }
/// ```
// TODO: add recursive parsing so the inner expression can be omitted
#[macro_export]
macro_rules! letters_for_vectors {
    ($($e:expr => [$($c:expr, $d:ident);+]),+) => {
        $(
            impl<T: MathComponent<T> + Copy> Vector<T, $e> {
                $(
                    #[inline]
                    pub const fn $d(&self) -> T {
                        self.data[$c]
                    }

                    paste! {
                        /// sets the `data[n]` to the value f
                        #[inline]
                        pub fn [<set_ $d>](&mut self, value: T) {
                            self.data[$c] = value;
                        }

                        // Creates a new unit vector from the corresponding letter
                        #[inline]
                        pub fn [<unit_ $d>]() -> Self {
                            Self::unit($c)
                        }
                    }

                )+
            }
        )+
    }
}
