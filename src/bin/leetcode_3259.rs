//! 超级饮料的最大强化能量

pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let len = energy_drink_a.len();
    let mut dp = vec![[0; 2]; len + 2];
    for ((i, a), b) in energy_drink_a.into_iter().enumerate().zip(energy_drink_b) {
        dp[i + 2][0] = dp[i + 1][0].max(dp[i][1]) + a as i64;
        dp[i + 2][1] = dp[i + 1][1].max(dp[i][0]) + b as i64;
    }
    dp[len + 1][0].max(dp[len + 1][1])
}

fn main() {
    fn test(func: fn(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64) {
        assert_eq!(func(vec![4, 1, 1], vec![1, 1, 3]), 7);
        assert_eq!(func(vec![1, 3, 1], vec![3, 1, 1]), 5);
    }
    test(max_energy_boost);
}
