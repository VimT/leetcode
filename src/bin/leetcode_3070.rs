//! 元素和小于等于 k 的子矩阵的数目

pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut pre = vec![0; n];
    let mut result = 0;
    for i in 0..m {
        let mut line = 0;
        for j in 0..n {
            pre[j] += grid[i][j];
            line += pre[j];
            if line <= k {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![7, 6, 3], vec![6, 6, 1]], 18), 4);
        assert_eq!(func(vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]], 20), 6);
    }
    test(count_submatrices);
}
