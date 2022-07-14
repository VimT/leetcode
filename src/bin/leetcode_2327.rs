//! 知道秘密的人数

/// dp[i] 表示第i天，一共有多少人知道了秘密。边差分边前缀和就行。
pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let n = n as usize;
    let delay = delay as usize;
    let forget = forget as usize;
    let mut dp = vec![0; n * 2 + 1];
    dp[0] = 1;
    for i in 0..n {
        if i > 0 {
            dp[i] = (dp[i] + dp[i - 1]) % MOD;
        }
        dp[i + delay] = (dp[i + delay] + dp[i]) % MOD;
        dp[i + forget] = (dp[i + forget] - dp[i] + MOD) % MOD;
    }
    (dp[n - 1] - if n >= 1 + forget { dp[n - 1 - forget] } else { 0 } + MOD) % MOD
}

fn main() {
    assert_eq!(people_aware_of_secret(684, 18, 496), 653668527);
    assert_eq!(people_aware_of_secret(6, 2, 4), 5);
    assert_eq!(people_aware_of_secret(4, 1, 3), 6);
}
