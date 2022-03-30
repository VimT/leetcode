//! 最低票价


pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    fn inner(days: &Vec<i32>, idx: usize, cost: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if idx >= days.len() { return 0; }
        if cache[idx] > 0 { return cache[idx]; }
        let today = days[idx];
        let mut i = idx;
        let buy_one = cost[0] + inner(days, i + 1, cost, cache);
        while i < days.len() && today + 7 > days[i] { i += 1; }
        let buy_7 = if i > idx { cost[1] + inner(days, i, cost, cache) } else { buy_one };
        while i < days.len() && today + 30 > days[i] { i += 1; }
        let buy_30 = if i > idx { cost[2] + inner(days, i, cost, cache) } else { buy_one };
        cache[idx] = buy_one.min(buy_7).min(buy_30);
        cache[idx]
    }
    inner(&days, 0, &costs, &mut vec![-1; days.len()])
}

/// dp[i] 为后n天的旅游费用
/// dp[i] = min( cost1 + dp[i - 1] , cost7 + dp[i_min7], cost30 + dp[i_min30])
pub fn mincost_tickets_dp(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let len = days.len();
    let mut dp = vec![0; len + 1];
    for i in 1..=len {
        let today = days[len - i];
        let mut idx_7 = 0;
        while len - i + idx_7 < days.len() && today + 7 > days[len - i + idx_7] { idx_7 += 1; }
        let mut idx_30 = 0;
        while len - i + idx_30 < days.len() && today + 30 > days[len - i + idx_30] { idx_30 += 1; }
        dp[i] = (costs[0] + dp[i - 1])
            .min(costs[1] + dp[i - idx_7])
            .min(costs[2] + dp[i - idx_30])
    }
    dp[len]
}


fn main() {
    fn test(func: fn(days: Vec<i32>, costs: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]), 17);
    }
    test(mincost_tickets);
    test(mincost_tickets_dp);
}
