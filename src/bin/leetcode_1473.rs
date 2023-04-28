//! 粉刷房子 III


pub fn min_cost(houses: Vec<i32>, mut cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    let len = m as usize;
    let n = n as usize;
    let target = target as usize;
    for i in 0..len {
        if houses[i] > 0 {
            cost[i].fill(i32::MAX / 2);
            cost[i][houses[i] as usize - 1] = 0;
        }
    }
    let mut dp = vec![vec![i32::MAX / 2; n]; target + 1];
    for i in 0..n {
        dp[1][i] = cost[0][i];
    }
    for i in 1..len {
        for j in (1..=target).rev() {
            for k in 0..n {
                if houses[i] != 0 && houses[i] - 1 != k as i32 {
                    dp[j][k] = i32::MAX / 2;
                    continue;
                }
                let mut tmp = i32::MAX;
                for bk in 0..n {
                    tmp = tmp.min(dp[j - (k != bk) as usize][bk] + cost[i][k]);
                }
                dp[j][k] = tmp;
            }
        }
    }
    let result = dp[target].iter().min().copied().unwrap();
    if result >= i32::MAX / 2 { -1 } else { result }
}

fn main() {
    fn test(func: fn(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32) {
        assert_eq!(func(vec![1], vec![vec![2]], 1, 1, 1), 0);
        assert_eq!(func(vec![0, 0, 0, 0, 0], vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]], 5, 2, 3), 9);
        assert_eq!(func(vec![0, 2, 1, 2, 0], vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]], 5, 2, 3), 11);
        assert_eq!(func(vec![3, 1, 2, 3], vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 4, 3, 3), -1);
    }
    test(min_cost);
}
