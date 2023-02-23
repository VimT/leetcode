//! 二进制矩阵中翻转最多一次使路径不连通

/// 找轮廓，（先右后下） vs (先下后右）。所有其他路径都在这个轮廓内
/// 如果这两条路径有交集，说明可以只翻转一次
pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return true;
        }
        grid[i][j] = 0;  // 标记访问过
        (i + 1 < grid.len() && grid[i + 1][j] == 1 && dfs(grid, i + 1, j)) ||
            (j + 1 < grid[0].len() && grid[i][j + 1] == 1 && dfs(grid, i, j + 1))
    }
    // 不可达，或者去掉一个路径后不可达
    !dfs(&mut grid, 0, 0) || !dfs(&mut grid, 0, 0)
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 0, 0], vec![1, 1, 1]]), true);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), false);
    }
    test(is_possible_to_cut_path);
}
