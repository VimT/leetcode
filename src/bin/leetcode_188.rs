//! 买卖股票的最佳时机 IV

/// dp[i][j][0]表示最多买卖j次，第i天结束时空仓的最高收益。
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let k = k as usize;
    let days = prices.len();
    if days == 0 || k == 0 { return 0; }
    if k >= days / 2 {
        let mut dp0 = 0;
        let mut dp1 = -prices[0];
        for i in 1..days {
            let tmp = dp0;
            dp0 = dp0.max(dp1 + prices[i]);
            dp1 = dp1.max(tmp - prices[i]);
        }
        return dp0.max(dp1);
    }
    let mut dp = vec![(0, 0); k + 1];
    for i in 0..=k {
        dp[i].1 = -prices[0];
    }
    for i in 1..=days {
        for j in 1..=k {
            dp[j].0 = dp[j].0.max(dp[j].1 + prices[i - 1]);
            dp[j].1 = dp[j].1.max(dp[j - 1].0 - prices[i - 1]);
        }
    }

    dp[k].0
}

fn main() {
    assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
    assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
}
