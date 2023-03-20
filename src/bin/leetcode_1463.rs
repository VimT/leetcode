//! 摘樱桃 II

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![i32::MIN; n]; n];
    dp[0][n - 1] = grid[0][0] + grid[0][n - 1];
    let mut dp2 = vec![vec![0; n]; n];
    for i in 1..m {
        for j in 0..n {
            for k in 0..n {
                let mut tmp = i32::MIN;
                for pj in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                    for pk in k.saturating_sub(1)..=(k + 1).min(n - 1) {
                        tmp = tmp.max(dp[pj][pk]);
                    }
                }
                dp2[j][k] = tmp + grid[i][j] + if j == k { 0 } else { grid[i][k] };
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    dp.into_iter().map(|x| x.into_iter().max().unwrap()).max().unwrap()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![4, 1, 5, 7, 1], vec![6, 0, 4, 6, 4], vec![0, 9, 6, 3, 5]]), 32);
        assert_eq!(func(vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]]), 24);
        assert_eq!(func(vec![vec![1, 0, 0, 0, 0, 0, 1], vec![2, 0, 0, 0, 0, 3, 0], vec![2, 0, 9, 0, 0, 0, 0], vec![0, 3, 0, 5, 4, 0, 0], vec![1, 0, 2, 3, 0, 0, 6]]), 28);
    }
    test(cherry_pickup);
}
