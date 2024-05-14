//! 判断矩阵是否满足条件

pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            if i > 0 && grid[i][j] != grid[i - 1][j] || j > 0 && grid[i][j] == grid[i][j - 1] {
                return false;
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![4, 4], vec![4, 4]]), false);
        assert_eq!(func(vec![vec![1, 0, 2], vec![1, 0, 2]]), true);
        assert_eq!(func(vec![vec![1, 1, 1], vec![0, 0, 0]]), false);
        assert_eq!(func(vec![vec![1], vec![2], vec![3]]), false);
    }
    test(satisfies_conditions);
}
