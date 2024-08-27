//! 最少翻转次数使二进制矩阵回文 I

pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut a = 0;
    for i in 0..m {
        for j in 0..n / 2 {
            if grid[i][j] != grid[i][n - j - 1] {
                a += 1;
            }
        }
    }

    let mut b = 0;
    for j in 0..n {
        for i in 0..m / 2 {
            if grid[i][j] != grid[m - i - 1][j] {
                b += 1;
            }
        }
    }
    a.min(b)
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), 2);
        assert_eq!(func(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 1);
        assert_eq!(func(vec![vec![1], vec![0]]), 0);
    }
    test(min_flips);
}
