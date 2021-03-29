use fructose::operators::mul_add::ClosedMulAdd;
use fructose::operators::{Additive, ClosedAdd, ClosedMul, ClosedRem, Multiplicative};
use fructose::properties::general::{Associative, Commutative, Identity, Set, Total};
use fructose::properties::helpers::bound::Bounded;
use fructose::properties::helpers::list::{ListSet, WholeListSet};
use std::ops::{Add, AddAssign, Mul, MulAssign, Range, Sub, SubAssign};

// TODO: add this to fructose
// this method is not intended to be used for 32bit and above sets

// Trait for Fixed Integers, mainly for ease of use
pub trait FixedInteger:
    Sized + ClosedAdd + ClosedMul + Bounded + ClosedMulAdd + ClosedRem + Copy
{
}
impl<T> FixedInteger for T where
    T: Sized + ClosedAdd + ClosedMul + Bounded + ClosedMulAdd + ClosedRem + Copy
{
}

// Fixed integer set e.g {0, 1, 2, 3, 4, 5} for MAX = 6;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct FI<T: FixedInteger, const MAX: usize> {
    val: T,
}

impl<T: FixedInteger, const MAX: usize> FI<T, { MAX }> {
    pub fn new(val: T) -> Self {
        Self { val }
    }

    pub fn value(&self) -> T {
        self.val
    }
}

macro_rules! impl_ops {
    ($($set_signed:ty : $set_unsigned:ty)*) => {
        $(
            impl_ops!(@general $set_signed);
            impl_ops!(@general $set_unsigned);
            impl_ops!(@signed $set_signed);
            impl_ops!(@unsigned $set_unsigned);
        )*
    };
    (@general $set:ty) => {
        impl<const MAX: usize> Add for FI<$set, { MAX }> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                let mut val = self.val + rhs.val;
                val %= MAX as $set;
                Self { val }
            }
        }

        impl<const MAX: usize> AddAssign for FI<$set, { MAX }> {
            fn add_assign(&mut self, rhs: Self) {
                self.val += rhs.val;
                self.val %= MAX as $set;
            }
        }

        impl<const MAX: usize> Mul for FI<$set, { MAX }> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut val = self.val * rhs.val;
                val %= MAX as $set;
                Self { val: val }
            }
        }

        impl<const MAX: usize> MulAssign for FI<$set, { MAX }> {
            fn mul_assign(&mut self, rhs: Self) {
                self.val *= rhs.val;
                self.val %= MAX as $set;
            }
        }

        impl<const MAX: usize> Set<Additive> for FI<$set, { MAX }> {
            fn operate(&self, rhs: Self) -> Self { *self + rhs }
        }

        impl<const MAX: usize> Total<Additive> for FI<$set, { MAX }> { }

        impl<const MAX: usize> Associative<Additive> for FI<$set, { MAX }> { }

        impl<const MAX: usize> Commutative<Additive> for FI<$set, { MAX }> { }

        // weird, the compiler is unable to figure out the $set type by himself?
        impl<const MAX: usize> Identity<Additive> for FI<$set, { MAX }> {
            fn identity() -> Self {
                Self {
                    val: <$set as Identity<Additive>>::identity()
                }
            }

            fn is_identity(&self) -> bool {
                *self == Self::identity()
            }
        }

        impl<const MAX: usize> Set<Multiplicative> for FI<$set, { MAX }> {
            fn operate(&self, rhs: Self) -> Self { *self * rhs }
        }

        impl<const MAX: usize> ListSet<$set> for FI<$set, { MAX }> {
            fn list_set(range: Range<$set>) -> Vec<$set> {
                let mut values = Vec::new();
                for val in range {
                    values.push(val);
                }
                values
            }
        }

        impl<const MAX: usize> WholeListSet<$set> for FI<$set, { MAX }> {
            fn whole_list_set() -> Vec<$set> {
                let mut values = Vec::new();
                for val in Self::MIN..(MAX as $set){
                    values.push(val);
                }
                values
            }
        }
    };
    (@signed $set:ty) => {
        impl<const MAX: usize> Bounded<$set> for FI<$set, { MAX }> {
            const MIN: $set = -(MAX as $set) + 1;
            const MAX: $set = (MAX as $set) - 1;
        }

        impl<const MAX: usize> Sub for FI<$set, { MAX }> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut val = self.val - rhs.val;
                val %= MAX as $set;
                Self { val }
            }
        }

        impl<const MAX: usize> SubAssign for FI<$set, { MAX }> {
            fn sub_assign(&mut self, rhs: Self) {
                self.val -= rhs.val;
                self.val %= MAX as $set;
            }
        }
    };
    (@unsigned $set:ty) => {
        impl<const MAX: usize> Bounded<$set> for FI<$set, { MAX }> {
            const MIN: $set = 0 as $set;
            const MAX: $set = (MAX - 1) as $set;
        }
    }
}

impl_ops!(i32: u32);

#[cfg(test)]
mod group_tests {
    use super::FI;
    use fructose::properties::helpers::bound::Bounded;
    use fructose::properties::helpers::list::{ListSet, WholeListSet};

    #[test]
    fn signed() {
        let x = FI::<i32, 6>::new(5);
        let y = FI::<i32, 6>::new(4);
        let z = x + y;
        let w = x * y;
        let a = x - y;
        let b = y - x;
        assert_eq!(z.value(), 3);
        assert_eq!(w.value(), 2);
        assert_eq!(a.value(), 1);
        assert_eq!(b.value(), -1);
        assert_eq!(FI::<i32, 6>::MIN, -5);
        assert_eq!(FI::<i32, 6>::MAX, 5);
    }

    #[test]
    fn unsigned() {
        let x = FI::<u32, 6>::new(5);
        let y = FI::<u32, 6>::new(4);
        let z = x + y;
        let w = x * y;
        assert_eq!(z.value(), 3);
        assert_eq!(w.value(), 2);
        assert_eq!(FI::<u32, 6>::MIN, 0);
        assert_eq!(FI::<u32, 6>::MAX, 5);
    }

    #[test]
    fn range() {
        let list_set_signed = FI::<i32, 5>::list_set((-2..4));
        let whole_list_set_signed = FI::<i32, 5>::whole_list_set();

        let list_set_unsigned = FI::<u32, 5>::list_set((1..3));
        let whole_list_set_unsigned = FI::<u32, 5>::whole_list_set();

        assert_eq!(list_set_signed, vec![-2, -1, 0, 1, 2, 3]);
        assert_eq!(whole_list_set_signed, vec![-4, -3, -2, -1, 0, 1, 2, 3, 4]);
        assert_eq!(list_set_unsigned, vec![1, 2]);
        assert_eq!(whole_list_set_unsigned, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn euclidean() {}
}

// // returns (coprime, order)
// pub fn order_multiplicative(modulo: i64, num: i64) -> (i64, i64) {
//     let mut order = 0;
//     let mut tmp = 1;
//     if num == 0 {
//         return (num, 1);
//     }
//     loop {
//         tmp *= num;
//         tmp %= modulo;
//         order += 1;
//         if tmp == 1 {
//             break;
//         }
//     }
//     (num, order)
// }
//
// pub fn order_multiplicative_2(modulo: i64, num: i64) -> (i64, i64) {
//     let mut order = 0;
//     let mut tmp = 1;
//     if num == 0 {
//         return (num, 1);
//     }
//     loop {
//         tmp *= num;
//         tmp %= modulo;
//         order += 1;
//         if tmp == 1 || order >= modulo {
//             break;
//         }
//     }
//     (num, order)
// }
//
// pub fn order_additive(modulo: i64, num: i64) -> (i64, i64) {
//     let mut order = 0;
//     let mut tmp = 1;
//     loop {
//         tmp += num;
//         tmp %= modulo;
//         order += 1;
//         if tmp == 1 {
//             break;
//         }
//     }
//     (num, order)
// }
//
// pub fn orders(modulo: i64, group: &[i64], kind: GroupType) -> Vec<(i64, i64)> {
//     match kind {
//         GroupType::Additive => group
//             .iter()
//             .map(|num| order_additive(modulo, *num))
//             .collect(),
//         GroupType::Multiplicative => group
//             .iter()
//             .map(|num| order_multiplicative_2(modulo, *num))
//             .collect(),
//         GroupType::MultiplicativeStar => group
//             .iter()
//             .map(|coprime| order_multiplicative(modulo, *coprime))
//             .collect(),
//     }
// }
//
// pub fn possible_orders(modulo: i64, kind: GroupType) -> Vec<i64> {
//     let mut possible_orders = Vec::new();
//     let group_size = group_size(modulo, kind);
//     for i in 1..group_size + 1 {
//         if group_size % i == 0 {
//             possible_orders.push(i);
//         }
//     }
//     possible_orders
// }
//
// pub fn producers(modulo: i64, group: &[i64], kind: GroupType, big: bool) -> Vec<i64> {
//     let group_size = group.len() as i64;
//     let mut producers = Vec::new();
//
//     match kind {
//         GroupType::Additive => {
//             let mut group_size_factors = wheel_factorization(group_size);
//             group_size_factors.dedup();
//             for e in group {
//                 let mut can_add = true;
//                 for factor in &group_size_factors {
//                     if e * (group_size / factor) % modulo == 1 {
//                         can_add = false;
//                         break;
//                     }
//                 }
//                 if can_add {
//                     producers.push(*e)
//                 }
//             }
//             let zero = producers.iter().position(|x| *x == 0);
//             if zero.is_some() {
//                 producers.remove(zero.unwrap());
//             }
//             if !producers.contains(&1) && group.contains(&1) {
//                 producers.insert(0, 1);
//             }
//         }
//         GroupType::Multiplicative => {
//             if big {
//                 group
//                     .iter()
//                     .filter(|num| is_producer_mul_2(modulo, **num, group_size))
//                     .for_each(|num| producers.push(*num))
//             } else {
//                 let mut group_size_factors = wheel_factorization(group_size);
//                 group_size_factors.dedup();
//                 for e in group {
//                     let mut can_add = true;
//                     for factor in &group_size_factors {
//                         if e.pow((group_size / factor) as u32) % modulo == 1 {
//                             can_add = false;
//                             break;
//                         }
//                     }
//                     if can_add {
//                         producers.push(*e)
//                     }
//                 }
//             }
//         }
//         GroupType::MultiplicativeStar => {
//             if big {
//                 group
//                     .iter()
//                     .filter(|num| is_producer_mul(modulo, **num, group_size))
//                     .for_each(|num| producers.push(*num))
//             } else {
//                 let mut group_size_factors = wheel_factorization(group_size);
//                 group_size_factors.dedup();
//                 for e in group {
//                     let mut can_add = true;
//                     for factor in &group_size_factors {
//                         if e.pow((group_size / factor) as u32) % modulo == 1 {
//                             can_add = false;
//                             break;
//                         }
//                     }
//                     if can_add {
//                         producers.push(*e)
//                     }
//                 }
//             }
//         }
//     }
//     producers
// }
//
// fn is_producer_mul(modulo: i64, num: i64, group_size: i64) -> bool {
//     let mut counter = 0;
//     let mut tmp = 1;
//     loop {
//         tmp *= num;
//         tmp %= modulo;
//         counter += 1;
//         if tmp == 1 {
//             break;
//         }
//     }
//     counter == group_size
// }
//
// fn is_producer_mul_2(modulo: i64, num: i64, group_size: i64) -> bool {
//     let mut counter = 0;
//     let mut tmp = 1;
//     loop {
//         tmp *= num;
//         tmp %= modulo;
//         counter += 1;
//         if tmp == 1 || counter >= modulo {
//             break;
//         }
//     }
//     counter == group_size
// }
//
// #[test]
// #[ignore]
// fn test() {
//     let modulo = 17;
//     let kind = GroupType::MultiplicativeStar;
//     let prime_factorization = wheel_factorization(modulo);
//     let group_size = group_size(modulo, kind);
//     let group_size_factorization = wheel_factorization(group_size);
//     let group = group(modulo, kind);
//     let possible_orders = possible_orders(modulo, kind);
//     let orders = orders(modulo, &group, kind);
//     let producers = producers(modulo, &group, kind, false);
//
//     println!("modulo: {}", modulo);
//     println!("group type: {:?}", kind);
//     println!("prime factors of n: {:?}", prime_factorization);
//     println!("group size: {} := {:?}", group_size, group);
//     println!("group size prime factors: {:?}", group_size_factorization);
//     println!("possible orders: {:?}", possible_orders);
//     println!("actual order: {:?}", orders);
//     println!("producers: {:?}", producers);
// }
