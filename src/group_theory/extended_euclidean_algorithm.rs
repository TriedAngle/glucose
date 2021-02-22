use crate::numeric::int::Int;
use std::ops::Neg;
use crate::num::num::NumAssignOps;
use std::fmt::Debug;
use crate::Matrix;
use crate::linear::dynamic::DMatrix;

/// input: a < b
pub fn extended_euclidean_algorithm<T: Int + NumAssignOps + Neg<Output = T>>(mut a: T, mut b: T) -> (T, T) {
    assert!(a < b);
    let mut ks = Vec::new();
    let mut k = T::zero();
    let mut a_tmp = T::zero();
    while b % a != T::zero() {
        k = b / a;
        ks.push(k);
        a_tmp = a;
        a = b - a * k;
        b = a_tmp;
    }
    let mut s = T::one();
    let mut t = T::zero();
    let mut s_temp = T::zero();
    while let Some(k) = ks.pop() {
        s_temp = s;
        s = k * s * -T::one() + t;
        t = s_temp
    }
    (s, t)
}

pub fn extended_euclidean_algorithm_with_steps<T: Int + NumAssignOps + Neg<Output = T>>(mut a: T, mut b: T)
    -> (Vec<(T, T)>, Vec<T>, Vec<(T, T)>) {
    assert!(a < b);
    let mut ks = Vec::new();
    let mut abs = vec![(a, b)];
    let mut k = T::zero();
    let mut a_tmp = T::zero();
    while b % a != T::zero() {
        k = b / a;
        ks.push(k);
        a_tmp = a;
        a = b - a * k;
        b = a_tmp;
        abs.push((a, b));
    }
    let mut s = T::one();
    let mut t = T::zero();
    let mut sts = vec![(s, t)];
    let mut s_temp = T::zero();
    let mut ks_clone = ks.clone();
    while let Some(k) = ks_clone.pop() {
        s_temp = s;
        s = k * s * -T::one() + t;
        t = s_temp;
        sts.push((s, t));
    }
    sts.reverse();
    (abs, ks, sts)
}

pub fn extended_euclidean_as_dmatrix<T: Int + NumAssignOps + Neg<Output = T>>(a: T, b: T) -> DMatrix<T>{
    let (abs, mut ks, sts) = extended_euclidean_algorithm_with_steps(a, b);
    ks.push(-T::one());
    let a_vec = abs.iter().map(|(a, b)| *a).collect();
    let b_vec = abs.iter().map(|(a, b)| *b).collect();
    let s_vec = sts.iter().map(|(s, t)| *s).collect();
    let t_vec = sts.iter().map(|(s, t)| *t).collect();
    let data = vec![a_vec, b_vec, ks, s_vec, t_vec];
    DMatrix::new(data)
}

#[cfg(test)]
mod euclidean_algo_tests {
    use super::*;

    #[test]
    fn algo() {
        let (a, b) = (935, 1491);
        let (s, t) = extended_euclidean_algorithm(a, b);

        assert_eq!(s, 716);
        assert_eq!(t, -449);
    }

    #[test]
    fn algo_with_steps() {
        let (a, b) = (935, 1491);
        let (abs, ks, sts) = extended_euclidean_algorithm_with_steps(a, b);
        println!("{:?}", abs);
        println!("{:?}", ks);
        println!("{:?}", sts);
    }

    #[test]
    fn algo_matrix() {
        let (a, b) = (935, 1491);
        let mat = extended_euclidean_as_dmatrix(a, b);
        println!("{:?}", mat);
        println!("{}", mat)
    }
}
