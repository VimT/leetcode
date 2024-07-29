//! 统计不是特殊数字的数字数量

use std::cmp::Ordering;
use std::sync::OnceLock;

use leetcode::algorithm::cal_prime;

pub fn non_special_count(l: i32, r: i32) -> i32 {
    static PRIME: OnceLock<Vec<usize>> = OnceLock::new();
    let prime = PRIME.get_or_init(|| {
        cal_prime(1e9_f64.sqrt() as usize + 1).into_iter().map(|x| x * x).collect()
    });
    let result = r + 1 - l;
    let ll = prime.binary_search_by(|x| x.cmp(&(l as usize)).then(Ordering::Greater)).unwrap_err();
    let rr = prime.binary_search_by(|x| x.cmp(&(r as usize)).then(Ordering::Less)).unwrap_err();
    result - (rr - ll) as i32
}

fn main() {
    fn test(func: fn(l: i32, r: i32) -> i32) {
        assert_eq!(func(5, 7), 3);
        assert_eq!(func(4, 16), 11);
    }
    test(non_special_count);
}
