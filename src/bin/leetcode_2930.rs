//! 重新排列后包含指定子字符串的字符串数目

use leetcode::algorithm::quick_pow;

/// 容斥原理：
/// 需要至少满足这3个条件的其中1个：1.不含 l， 2.不含 t。 3.不含 e 或者恰好包含一个 e。
/// 所以 result = 26^n - 满足一个条件的 + 满足两个条件 - 满足三个条件
pub fn string_count(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let result = quick_pow(26, n, MOD)
        - (3 * 25 + n) * quick_pow(25, n - 1, MOD)
        + (3 * 24 + 2 * n) * quick_pow(24, n - 1, MOD)
        - (23 + n) * quick_pow(23, n - 1, MOD);
    result.rem_euclid(MOD) as i32
}

/// DP
pub fn string_count2(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut dp = vec![[[[0; 3]; 2]; 2]; n + 1]; // dp[i][l][t][e] 表示前 i 个字符串，包含 l 个 l，t 个 t，e 个 e 的方案数
    dp[0][0][0][0] = 1;
    for i in 1..=n {
        for l in 0..=1 {
            for t in 0..=1 {
                for e in 0..=2 {
                    let mut sum = dp[i - 1][0][t][e];
                    sum += dp[i - 1][l][0][e];
                    sum += dp[i - 1][l][t][if e > 0 { e - 1 } else { 0 }];
                    sum += dp[i - 1][l][t][e] * 23;
                    dp[i][l][t][e] = sum % MOD;
                }
            }
        }
    }
    dp[n][1][1][2] as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(4), 12);
        assert_eq!(func(10), 83943898);
    }
    test(string_count);
    test(string_count2);
}
