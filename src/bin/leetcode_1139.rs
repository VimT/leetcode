//! 最大的以 1 为边界的正方形

pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut left = vec![vec![0; n + 1]; m + 1];
    let mut up = vec![vec![0; n + 1]; m + 1];
    let mut result = 0;
    for i in 1..=m {
        for j in 1..=n {
            if grid[i - 1][j - 1] == 1 {
                left[i][j] = left[i][j - 1] + 1;
                up[i][j] = up[i - 1][j] + 1;
                for k in 1..=left[i][j].min(up[i][j]) {
                    if up[i][j - k + 1] >= k && left[i - k + 1][j] >= k {
                        result = result.max(k);
                    }
                }
            }
        }
    }
    (result * result) as i32
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), 9);
        assert_eq!(func(vec![vec![1, 1, 0, 0]]), 1);
    }
    test(largest1_bordered_square);
}
