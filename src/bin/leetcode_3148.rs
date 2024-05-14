//! 矩阵中的最大得分

pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut pre_min = vec![i32::MAX / 2; n]; // 前n列的最小值
    let mut result = i32::MIN;
    for i in 0..m {
        for j in 0..n {
            result = result.max(grid[i][j] - pre_min[j].min(pre_min[j.saturating_sub(1)]));
            pre_min[j] = pre_min[j].min(grid[i][j]);
            if j > 0 { pre_min[j] = pre_min[j].min(pre_min[j - 1]) }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![4, 3], vec![2, 3]]), 1);
        assert_eq!(func(vec![vec![9, 5, 7, 3], vec![8, 9, 6, 1], vec![6, 7, 14, 3], vec![2, 5, 3, 1]]), 9);
        assert_eq!(func(vec![vec![4, 3, 2], vec![3, 2, 1]]), -1);
    }
    test(max_score);
}
