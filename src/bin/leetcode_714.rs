//! 买卖股票的最佳时机含手续费

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let len = prices.len();
    let mut dp = vec![[0, 0]; len];
    dp[0][1] = -prices[0];
    for i in 1..len {
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + prices[i] - fee);
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] - prices[i]);
    }
    dp[len - 1][0]
}

fn main() {
    assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    assert_eq!(max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
}
