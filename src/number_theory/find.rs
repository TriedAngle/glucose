// TODO: generify
// TODO: use EA for this
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

// TODO: generify
pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
