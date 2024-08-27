//! 最少翻转次数使二进制矩阵回文 II

pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    for i in 0..m / 2 {
        for j in 0..n / 2 {
            let a = grid[i][j];
            let b = grid[m - 1 - i][j];
            let c = grid[i][n - 1 - j];
            let d = grid[m - 1 - i][n - 1 - j];
            result += match a + b + c + d {
                1 => 1,
                2 => 2,
                3 => 1,
                _ => 0
            };
        }
    }
    let mut one = 0;
    let mut diff = 0;
    if m & 1 == 1 {
        for j in 0..n / 2 {
            let a = grid[m / 2][j];
            let b = grid[m / 2][n - 1 - j];
            if a != b {
                diff += 1;
            } else if a == 1 {
                one += 2;
            }
        }
    }
    if n & 1 == 1 {
        for i in 0..m / 2 {
            let a = grid[i][n / 2];
            let b = grid[m - 1 - i][n / 2];
            if a != b {
                diff += 1;
            } else if a == 1 {
                one += 2;
            }
        }
    }
    if one % 4 == 2 && diff == 0 {
        diff += 2;
    }
    result += diff;
    if m & 1 == 1 && n & 1 == 1 {
        result += grid[m / 2][n / 2];
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 0, 1], vec![1, 0, 0], vec![0, 1, 1]]), 5);
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 0, 1], vec![0, 0, 1]]), 3);
        assert_eq!(func(vec![vec![1], vec![1]]), 2);
        assert_eq!(func(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]]), 4);
        assert_eq!(func(vec![vec![1, 0, 1], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), 1);
        assert_eq!(func(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 2);
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    }
    test(min_flips);
}
