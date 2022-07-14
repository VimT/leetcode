//! 寻找数组的错位排列

/// f(n) = (n - 1) * (f(n - 1) + f(n - 2))
pub fn find_derangement(n: i32) -> i32 {
    const MOD: usize = 1e9 as usize + 7;
    if n == 0 { return 1; }
    if n == 1 { return 0; }
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 1;
    for i in 2..=n as usize {
        dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2]) % MOD;
    }
    dp[n as usize] as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(3), 2);
        assert_eq!(func(2), 1);
    }
    test(find_derangement);
}
