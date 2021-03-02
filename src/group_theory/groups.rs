use crate::number_theory::prime::{coprimes, wheel_factorization};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GroupType {
    Additive,
    Multiplicative,
    MultiplicativeStar,
}

pub fn group_size(modulo: i64, kind: GroupType) -> i64 {
    match kind {
        GroupType::Additive => modulo,
        GroupType::Multiplicative => modulo,
        GroupType::MultiplicativeStar => coprimes(modulo).len() as i64,
    }
}

pub fn group(modulo: i64, kind: GroupType) -> Vec<i64> {
    match kind {
        GroupType::Additive => (0..modulo).into_iter().collect(),
        GroupType::Multiplicative => (0..modulo).into_iter().collect(),
        GroupType::MultiplicativeStar => coprimes(modulo),
    }
}

// returns (coprime, order)
pub fn order_multiplicative(modulo: i64, num: i64) -> (i64, i64) {
    let mut order = 0;
    let mut tmp = 1;
    if num == 0 {
        return (num, 1);
    }
    loop {
        tmp *= num;
        tmp %= modulo;
        order += 1;
        if tmp == 1 {
            break;
        }
    }
    (num, order)
}

pub fn order_multiplicative_2(modulo: i64, num: i64) -> (i64, i64) {
    let mut order = 0;
    let mut tmp = 1;
    if num == 0 {
        return (num, 1);
    }
    loop {
        tmp *= num;
        tmp %= modulo;
        order += 1;
        if tmp == 1 || order >= modulo {
            break;
        }
    }
    (num, order)
}

pub fn order_additive(modulo: i64, num: i64) -> (i64, i64) {
    let mut order = 0;
    let mut tmp = 1;
    loop {
        tmp += num;
        tmp %= modulo;
        order += 1;
        if tmp == 1 {
            break;
        }
    }
    (num, order)
}

pub fn orders(modulo: i64, group: &[i64], kind: GroupType) -> Vec<(i64, i64)> {
    match kind {
        GroupType::Additive => group
            .iter()
            .map(|num| order_additive(modulo, *num))
            .collect(),
        GroupType::Multiplicative => group
            .iter()
            .map(|num| order_multiplicative_2(modulo, *num))
            .collect(),
        GroupType::MultiplicativeStar => group
            .iter()
            .map(|coprime| order_multiplicative(modulo, *coprime))
            .collect(),
    }
}

pub fn possible_orders(modulo: i64, kind: GroupType) -> Vec<i64> {
    let mut possible_orders = Vec::new();
    let group_size = group_size(modulo, kind);
    for i in 1..group_size + 1 {
        if group_size % i == 0 {
            possible_orders.push(i);
        }
    }
    possible_orders
}

pub fn producers(modulo: i64, group: &[i64], kind: GroupType, big: bool) -> Vec<i64> {
    let group_size = group.len() as i64;
    let mut producers = Vec::new();

    match kind {
        GroupType::Additive => {
            let mut group_size_factors = wheel_factorization(group_size);
            group_size_factors.dedup();
            for e in group {
                let mut can_add = true;
                for factor in &group_size_factors {
                    if e * (group_size / factor) % modulo == 1 {
                        can_add = false;
                        break;
                    }
                }
                if can_add {
                    producers.push(*e)
                }
            }
            let zero = producers.iter().position(|x| *x == 0);
            if zero.is_some() {
                producers.remove(zero.unwrap());
            }
            if !producers.contains(&1) && group.contains(&1) {
                producers.insert(0, 1);
            }
        }
        GroupType::Multiplicative => {
            if big {
                group
                    .iter()
                    .filter(|num| is_producer_mul_2(modulo, **num, group_size))
                    .for_each(|num| producers.push(*num))
            } else {
                let mut group_size_factors = wheel_factorization(group_size);
                group_size_factors.dedup();
                for e in group {
                    let mut can_add = true;
                    for factor in &group_size_factors {
                        if e.pow((group_size / factor) as u32) % modulo == 1 {
                            can_add = false;
                            break;
                        }
                    }
                    if can_add {
                        producers.push(*e)
                    }
                }
            }
        }
        GroupType::MultiplicativeStar => {
            if big {
                group
                    .iter()
                    .filter(|num| is_producer_mul(modulo, **num, group_size))
                    .for_each(|num| producers.push(*num))
            } else {
                let mut group_size_factors = wheel_factorization(group_size);
                group_size_factors.dedup();
                for e in group {
                    let mut can_add = true;
                    for factor in &group_size_factors {
                        if e.pow((group_size / factor) as u32) % modulo == 1 {
                            can_add = false;
                            break;
                        }
                    }
                    if can_add {
                        producers.push(*e)
                    }
                }
            }
        }
    }
    producers
}

fn is_producer_mul(modulo: i64, num: i64, group_size: i64) -> bool {
    let mut counter = 0;
    let mut tmp = 1;
    loop {
        tmp *= num;
        tmp %= modulo;
        counter += 1;
        if tmp == 1 {
            break;
        }
    }
    counter == group_size
}

fn is_producer_mul_2(modulo: i64, num: i64, group_size: i64) -> bool {
    let mut counter = 0;
    let mut tmp = 1;
    loop {
        tmp *= num;
        tmp %= modulo;
        counter += 1;
        if tmp == 1 || counter >= modulo {
            break;
        }
    }
    counter == group_size
}

#[test]
#[ignore]
fn test() {
    let modulo = 17;
    let kind = GroupType::MultiplicativeStar;
    let prime_factorization = wheel_factorization(modulo);
    let group_size = group_size(modulo, kind);
    let group_size_factorization = wheel_factorization(group_size);
    let group = group(modulo, kind);
    let possible_orders = possible_orders(modulo, kind);
    let orders = orders(modulo, &group, kind);
    let producers = producers(modulo, &group, kind, false);

    println!("modulo: {}", modulo);
    println!("group type: {:?}", kind);
    println!("prime factors of n: {:?}", prime_factorization);
    println!("group size: {} := {:?}", group_size, group);
    println!("group size prime factors: {:?}", group_size_factorization);
    println!("possible orders: {:?}", possible_orders);
    println!("actual order: {:?}", orders);
    println!("producers: {:?}", producers);
}
