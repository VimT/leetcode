//! 骑士拨号器

pub fn knight_dialer(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut dp = vec![1; 10];
    for _ in 1..n {
        let mut new_dp = vec![0; 10];
        new_dp[1] = (dp[6] + dp[8]) % MOD;
        new_dp[2] = (dp[7] + dp[9]) % MOD;
        new_dp[3] = (dp[8] + dp[4]) % MOD;
        new_dp[4] = (dp[3] + dp[9] + dp[0]) % MOD;
        new_dp[6] = (dp[1] + dp[7] + dp[0]) % MOD;
        new_dp[7] = (dp[2] + dp[6]) % MOD;
        new_dp[8] = (dp[1] + dp[3]) % MOD;
        new_dp[9] = (dp[2] + dp[4]) % MOD;
        new_dp[0] = (dp[6] + dp[4]) % MOD;
        dp = new_dp;
    }
    (dp.into_iter().sum::<i64>() % MOD) as i32
}

fn main() {
    assert_eq!(knight_dialer(1), 10);
    assert_eq!(knight_dialer(2), 20);
    assert_eq!(knight_dialer(3131), 136006598);
}
