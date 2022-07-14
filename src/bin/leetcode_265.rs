//! 粉刷房子 II

pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
    let len = costs.len();
    let k = costs[0].len();
    let mut dp = vec![0; k];
    for i in 0..len {
        let mut ndp = vec![i32::MAX; k];
        for j in 0..k {
            for k in 0..k {
                if j != k {
                    ndp[j] = ndp[j].min(dp[k] + costs[i][j]);
                }
            }
        }
        dp = ndp
    }
    *dp.iter().min().unwrap()
}

fn main() {
    fn test(func: fn(costs: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 5, 3], vec![2, 9, 4]]), 5);
        assert_eq!(func(vec![vec![1, 3], vec![2, 4]]), 5);
    }
    test(min_cost_ii);
}
