//! 购买水果需要的最少金币数

use std::collections::VecDeque;

pub fn minimum_coins(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut dp = vec![i32::MAX; len + 1];  // dp[i] 表示前 i 个水果需要的最少金币数
    dp[0] = 0;
    for i in 0..len {
        dp[i + 1] = dp[i / 2..=i].iter().enumerate().map(|(i, &x)| prices[i] + x).min().unwrap();
    }
    dp[len]
}

/// 单调队列
/// DP的做法，相当于是求一个滑动窗口的最小值，可以用单调队列优化
pub fn minimum_coins2(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut q: VecDeque<(usize, i32)> = VecDeque::new();  // (i, val) 表示覆盖前 i 个水果且最后一个水果是第 i-1 个水果时，需要的最少金币数
    for i in 0..=len {
        let new_val = q.front().map(|&x| x.1 + prices[i - 1]).unwrap_or(0);
        while !q.is_empty() && q.back().unwrap().1 >= new_val {
            q.pop_back();
        }
        q.push_back((i, new_val));
        while !q.is_empty() && q.front().unwrap().0 < (i + 1) / 2 {
            q.pop_front();
        }
    }
    q.front().unwrap().1
}

fn main() {
    fn test(func: fn(prices: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 1, 2]), 4);
        assert_eq!(func(vec![1, 10, 1, 1]), 2);
    }
    test(minimum_coins);
    test(minimum_coins2);
}
