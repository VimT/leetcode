//! 统计放置房子的方式数

pub fn count_house_placements(n: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut dp = [1; 4];
    for _ in 1..n {
        let mut new_dp = [0; 4];
        new_dp[0] = (dp[0] + dp[1] + dp[2] + dp[3]) % MOD;
        new_dp[1] = (dp[0] + dp[2]) % MOD;
        new_dp[2] = (dp[0] + dp[1]) % MOD;
        new_dp[3] = dp[0];
        dp = new_dp;
    }
    (dp.iter().sum::<i64>() % MOD) as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(1000), 500478595);
        assert_eq!(func(1), 4);
        assert_eq!(func(2), 9);
    }
    test(count_house_placements);
}
