//! 销售利润最大化

pub fn maximize_the_profit(n: i32, mut offers: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1]; // dp[i] 表示卖出前i个房屋的最大利润
    offers.sort_unstable_by_key(|x| x[1]);
    let mut o = 0;
    let len = offers.len();
    for i in 1..=n {
        dp[i] = dp[i - 1];
        while o < len && offers[o][1] + 1 <= i as i32 {
            dp[i] = dp[i].max(dp[offers[o][0] as usize] + offers[o][2]);
            o += 1;
        }
    }
    dp[n]
}

fn main() {
    fn test(func: fn(n: i32, offers: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]]), 3);
        assert_eq!(func(5, vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]]), 10);
    }
    test(maximize_the_profit);
}
