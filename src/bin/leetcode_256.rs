//! 粉刷房子

pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut dp = [0; 3];
    let len = costs.len();
    for i in 0..len {
        let mut ndp = [i32::MAX; 3];
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    ndp[j] = ndp[j].min(dp[k] + costs[i][j]);
                }
            }
        }
        dp = ndp
    }
    dp[0].min(dp[1]).min(dp[2])
}

fn main() {
    fn test(func: fn(costs: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]), 10);
        assert_eq!(func(vec![vec![7, 6, 2]]), 2);
    }
    test(min_cost);
}
