//! 统计构造好字符串的方案数

pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let high = high as usize;
    let low = low as usize;
    let zero = zero as usize;
    let one = one as usize;
    let mut dp = vec![0; high + 1];
    const MOD: i32 = 1e9 as i32 + 7;
    dp[0] = 1;
    for i in 1..=high {
        if i >= zero {
            dp[i] += dp[i - zero];
        }
        if i >= one {
            dp[i] += dp[i - one];
        }
        dp[i] %= MOD;
    }
    dp[low..=high].iter().fold(0, |result, num| (result + *num) % MOD)
}

fn main() {
    assert_eq!(count_good_strings(3, 3, 1, 1), 8);
    assert_eq!(count_good_strings(2, 3, 1, 2), 5);
}
