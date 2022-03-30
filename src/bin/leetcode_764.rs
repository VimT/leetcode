//! 最大加号标志


pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = vec![vec![1; n]; n];
    for mine in mines {
        grid[mine[0] as usize][mine[1] as usize] = 0;
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        let mut one_cnt = 0;
        for j in 0..n {
            if grid[i][j] == 0 { one_cnt = 0; } else { one_cnt += 1 };
            dp[i][j] = one_cnt;
        }

        one_cnt = 0;
        for j in (0..n).rev() {
            if grid[i][j] == 0 { one_cnt = 0; } else { one_cnt += 1; }
            if one_cnt < dp[i][j] { dp[i][j] = one_cnt; }
        }
    }
    let mut result = 0;
    for j in 0..n {
        let mut one_cnt = 0;
        for i in 0..n {
            if grid[i][j] == 0 { one_cnt = 0; } else { one_cnt += 1; }
            if one_cnt < dp[i][j] { dp[i][j] = one_cnt; }
        }
        one_cnt = 0;
        for i in (0..n).rev() {
            if grid[i][j] == 0 { one_cnt = 0; } else { one_cnt += 1; }
            result = result.max(one_cnt.min(dp[i][j]));
        }
    }
    result
}

fn main() {
    assert_eq!(order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
    assert_eq!(order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
}
