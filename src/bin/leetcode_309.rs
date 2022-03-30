//! 最佳买卖股票时机含冷冻期

/// f(x) = >i天买入所获得的最大利润
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut dp = vec![0; len];
    for i in (0..len - 1).rev() {
        let buy_in = prices[i];
        let mut max = dp[i + 1];
        for j in i + 1..len {
            let after = if j + 2 < len { dp[j + 2] } else { 0 };
            max = max.max(prices[j] - buy_in + after);
        }
        dp[i] = max;
        //println!("{}: {}", i, max);
    }
    dp[0]
}

fn main() {
    assert_eq!(max_profit(vec![6, 1, 6, 4, 3, 0, 2]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
}
