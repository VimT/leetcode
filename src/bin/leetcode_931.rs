//! 下降路径最小和

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let len = matrix.len();
    let mut dp = vec![0; len];
    for line in matrix {
        let mut new_dp = vec![0; len];
        for (i, &num) in line.iter().enumerate() {
            new_dp[i] = num + *dp[(i as i32 - 1).max(0) as usize..=(i + 1).min(len - 1)].iter().min().unwrap();
        }
        dp = new_dp;
    }
    *dp.iter().min().unwrap()
}

fn main() {
    assert_eq!(min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]), 13);
    assert_eq!(min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]), -59);
}
