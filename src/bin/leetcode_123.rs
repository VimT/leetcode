//! 买卖股票的最佳时机 III

/// 模板题，dp[i][k][action] 第i天还能k次交易的时候，action 0不持有，1持有
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    if len == 0 { return 0; }
    let mut dp = vec![vec![(0_i32, 0_i32); 3]; len];
    for i in 0..len {
        for k in (1..=2).rev() {
            if i == 0 {
                dp[i][k].1 = -prices[0];
            } else {
                dp[i][k].0 = dp[i - 1][k].0.max(dp[i - 1][k].1 + prices[i]);
                dp[i][k].1 = dp[i - 1][k].1.max(dp[i - 1][k - 1].0 - prices[i]);
            }
        }
    }

    return dp[len - 1][2].0;
}

fn main() {
    assert_eq!(max_profit(vec![3, 3, 5, 0, 3, 1, 4]), 6);
}