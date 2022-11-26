//! 沙漏的最大总和

pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    for i in 0..m - 2 {
        for j in 0..n - 2 {
            let mut sum = 0;
            for x in i..i + 3 {
                for y in j..j + 3 {
                    sum += grid[x][y];
                }
            }
            result = result.max(sum - grid[i + 1][j] - grid[i + 1][j + 2]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![6, 2, 1, 3], vec![4, 2, 1, 5], vec![9, 2, 8, 7], vec![4, 1, 2, 9]]), 30);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 35);
    }
    test(max_sum);
}
