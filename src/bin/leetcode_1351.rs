//! 统计有序矩阵中的负数

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut j = (0..n).find(|&x| grid[0][x] < 0).unwrap_or(n);
    let mut result = n - j;
    for i in 1..m {
        while j > 0 && grid[i][j - 1] < 0 {
            j -= 1;
        }
        result += n - j;
    }
    result as i32
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]), 8);
        assert_eq!(func(vec![vec![3, 2], vec![1, 0]]), 0);
    }
    test(count_negatives);
}
