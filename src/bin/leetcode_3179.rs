//! K 秒后第 N 个元素的值

use leetcode::algorithm::combination_num;

pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut nums = vec![1; n];
    const MOD: i32 = 1_000_000_007;
    for _ in 0..k {
        let mut cur = 1;
        for j in 1..n {
            cur = (cur + nums[j]) % MOD;
            nums[j] = cur;
        }
    }
    nums[n - 1]
}

/// 相当于计算的是杨辉三角第 n+k 排的第 n 个数
pub fn value_after_k_seconds2(n: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let c = combination_num(2000, MOD);
    c((n + k - 1) as i64, k as i64) as i32
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(4, 5), 56);
        assert_eq!(func(5, 3), 35);
    }
    test(value_after_k_seconds);
    test(value_after_k_seconds2);
}
