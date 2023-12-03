//! 统计感冒序列的数目

use leetcode::algorithm::{combination_num, quick_pow};

/// 组合数学
pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let c = combination_num(1000, MOD);
    let mut b = vec![sick[0] as i64, (n - sick[sick.len() - 1] - 1) as i64];
    let mut result = 1;
    for win in sick.windows(2) {
        let size = (win[1] - win[0] - 1) as i64;
        if size > 0 {
            b.push(size);
            result = result * quick_pow(2, size - 1, MOD) % MOD;
        }
    }
    let mut total: i64 = b.iter().sum();
    for v in b {
        result = result * c(total, v) % MOD;
        total -= v;
    }
    result as i32
}

fn main() {
    fn test(func: fn(n: i32, sick: Vec<i32>) -> i32) {
        assert_eq!(func(5, vec![0, 4]), 4);
        assert_eq!(func(4, vec![1]), 3);
    }
    test(number_of_sequence);
}
