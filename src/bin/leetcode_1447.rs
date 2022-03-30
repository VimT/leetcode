//! 最简分数

use leetcode::svec;

#[inline]
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut result = vec![];
    for fenmu in 2..=n {
        for fenzi in 1..fenmu {
            if gcd(fenzi, fenmu) == 1 {
                result.push(format!("{}/{}", fenzi, fenmu));
            }
        }
    }
    result
}

fn main() {
    assert_eq!(simplified_fractions(2), svec!["1/2"]);
    assert_eq!(simplified_fractions(3), svec!["1/2", "1/3", "2/3"]);
    assert_eq!(simplified_fractions(4), svec!["1/2", "1/3", "2/3", "1/4", "3/4"]);
}
