use crate::num::int::Int;
use crate::num::num::NumAssignOps;

// TODO: generify
pub fn wheel_factorization(mut num: i64) -> Vec<i64> {
    let mut primes = Vec::new();
    let wheel: [i64; 11] = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
    let (mut f, mut w) = (2, 0);

    while f * f <= num {
        if num % f == 0 {
            primes.push(f);
            num /= f;
        } else {
            f += wheel[w];
            w = if w == 10 { 3 } else { w + 1 };
        }
    }
    primes.push(num);
    primes
}

pub fn is_prime(num: i64) -> bool {
    if num <= 3 && num > 1 {
        true
    } else if num % 2 == 0 || num % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= num {
            if num % i == 0 || num % i + 2 == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    let mut i = 1;
    let mut gcd = 1;
    while i <= a && i <= b {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
        i += 1;
    }
    return gcd;
}

pub fn coprimes(modulo: i64) -> Vec<i64> {
    let mut coprimes = Vec::new();
    for i in 1..modulo {
        if gcd(i, modulo) == 1 {
            coprimes.push(i);
        }
    }
    coprimes
}

// returns (coprime, order)
pub fn order_multiplicative(modulo: i64, coprime: i64) -> (i64, i64) {
    let mut order = 0;
    let mut tmp = 1;
    loop {
        tmp *= coprime;
        tmp %= modulo;
        order += 1;
        if tmp == 1 {
            break;
        }
    }
    (coprime, order)
}

//TODO
pub fn order_additive(modulo: i64, coprime: i64) -> (i64, i64) {
    unimplemented!()
}

pub fn orders(modulo: i64, coprimes: &[i64], multiplicative: bool) -> Vec<(i64, i64)> {
    match multiplicative {
        true => coprimes
            .iter()
            .map(|coprime| order_multiplicative(modulo, *coprime))
            .collect(),
        false => coprimes
            .iter()
            .map(|coprime| order_additive(modulo, *coprime))
            .collect(),
    }
}

// pub fn producers(Vec)

#[test]
fn test() {
    // println!("{:?}", wheel_factorization(743094734));
    // println!("{:?}", is_prime(677533));
    let coprimes = coprimes(225);
    let orders = orders(225, &coprimes, true);
    println!("{:?}, {}", coprimes, coprimes.len());
    println!("{:?}", orders);
}
