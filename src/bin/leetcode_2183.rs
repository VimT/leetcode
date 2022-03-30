//! 统计可以被 K 整除的下标对数目

use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
    let mut divisors = vec![];
    let mut d = 1;
    while d * d <= k {
        if k % d == 0 {
            divisors.push(d);
            if d * d < k {
                divisors.push(k / d);
            }
        }
        d += 1;
    }
    let mut result = 0;
    let mut cnt = HashMap::new();
    for v in nums {
        result += *cnt.get(&(k / gcd(k, v))).unwrap_or(&0);
        for &d in &divisors {
            if v % d == 0 {
                *cnt.entry(d).or_insert(0i64) += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(count_pairs(vec![1, 2, 3, 4, 5], 2), 7);
    assert_eq!(count_pairs(vec![1, 2, 3, 4], 5), 0);
}
