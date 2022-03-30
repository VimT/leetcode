//! 完美数

/// 欧几里得-欧拉定理
pub fn check_perfect_number(num: i32) -> bool {
    const PRIMES: [i64; 8] = [2, 3, 5, 7, 13, 17, 19, 31];
    for prime in PRIMES {
        if (1 << (prime - 1)) * ((1 << prime) - 1) == num as i64 {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(check_perfect_number(28), true);
    assert_eq!(check_perfect_number(7), false);
}
