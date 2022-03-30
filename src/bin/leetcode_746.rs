//! 使用最小花费爬楼梯

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let len = cost.len();
    let mut dp = vec![i32::MAX; len + 1];
    dp[0] = 0;
    dp[1] = 0;
    for i in 2..=len {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }
    dp[len]
}

fn main() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
}
