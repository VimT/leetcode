//! 字符串转换

use leetcode::algorithm::{matrix_power, max_match_length, quick_pow};

/// dp --> 矩阵快速幂
/// c 表示 s 在 (c+t) 的出现次数（KMP计算）
/// dp[i][0] 表示i次操作后，等于s的方案数，dp[i][1] 表示i次操作后，不等于s的方案数
///   dp[i][0] = dp[i-1][0] * (c-1) + dp[i-1][1] * c
///   dp[i][1] = dp[i-1][0] * (n-c) + dp[i-1][1] * (n-c-1)
/// 写成矩阵的形式
///   dp[i][0] = dp[i-1][0] * c-1    c
///   dp[i][1]   dp[i-1][1]   n-c  n-c-1
/// 所以
///   dp[k][0] = c-1    c   ^ k  *  1
///   dp[k][1]   n-c  n-c-1         0
pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let s = s.as_bytes();
    let t = t.as_bytes();
    let next = max_match_length(t);
    let mut j = 0;
    let mut c = 0;
    let len = s.len();
    for &ch in s.iter().chain(&s[..len - 1]) {
        while j > 0 && t[j] != ch { j = next[j - 1]; }
        if t[j] == ch { j += 1; }
        if j == len {
            c += 1;
            j = next[j - 1];
        }
    }
    let n = s.len() as i64;
    let m = vec![
        vec![c - 1, c],
        vec![n - c, n - c - 1],
    ];
    let result = matrix_power(m, k, MOD);
    (if s == t { result[0][0] } else { result[0][1] }) as i32
}

/// dp[i][j] 表示第i次操作后，偏移量为j的方案数
/// dp[i][j] = sum[0<=j'<n,j'!=j] (dp[i-1][j']) = Si-1 - dp[i-1][j]
/// Si 表示 sum(dp[i])，经过推导可以得到 Si = (n-1)^k * S0
/// 我们想要的结果 dp[k][0] = (-1)^k * (dp[0][0] + ((1-n)^k - 1) / n * S0)
pub fn number_of_ways2(s: String, t: String, k: i64) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let s = s.as_bytes();
    let t = t.as_bytes();
    let next = max_match_length(t);
    let mut j = 0;
    let mut c = 0;
    let len = s.len();
    for &ch in s.iter().chain(&s[..len - 1]) {
        while j > 0 && t[j] != ch { j = next[j - 1]; }
        if t[j] == ch { j += 1; }
        if j == len {
            c += 1;
            j = next[j - 1];
        }
    }
    let n = s.len() as i64;
    let mut result = (quick_pow(1 - n, k, MOD) - 1) * quick_pow(n, MOD - 2, MOD) % MOD * c % MOD;
    if s == t { result = (result + 1) % MOD; }
    if k & 1 == 1 { result = (MOD - result) % MOD }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, t: String, k: i64) -> i32) {
        assert_eq!(func(String::from("hmwfh"), String::from("hhmwf"), 10), 209715);
        assert_eq!(func(String::from("abcd"), String::from("cdab"), 2), 2);
        assert_eq!(func(String::from("goxoq"), String::from("dfqgl"), 244326024901249), 0);
        assert_eq!(func(String::from("ababab"), String::from("ababab"), 1), 2);
    }
    test(number_of_ways);
    test(number_of_ways2);
}
