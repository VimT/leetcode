//! 多米诺和托米诺平铺

/// dp[state] 表示当前列在不同状态下铺砖方式的数量。
/// dp[0] 表示当前列两行都不铺；
/// dp[1] 表示当前列第一行不铺，第二行铺；
/// dp[2] 表示当前列第一行铺，第二行不铺；
/// dp[3] 表示当前列的两行都铺
pub fn num_tilings(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut dp = [1i64, 0, 0, 0];
    for _ in 0..n {
        let mut ndp = [1i64, 0, 0, 0];
        ndp[0b00] = (dp[0b00] + dp[0b11]) % MOD;
        ndp[0b01] = (dp[0b00] + dp[0b10]) % MOD;
        ndp[0b10] = (dp[0b00] + dp[0b01]) % MOD;
        ndp[0b11] = (dp[0b00] + dp[0b01] + dp[0b10]) % MOD;
        dp = ndp;
    }
    dp[0] as i32
}

fn main() {
    assert_eq!(num_tilings(3), 5);
    assert_eq!(num_tilings(1), 1);
}
