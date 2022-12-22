//! 删除每行中的最大值

pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    for row in &mut grid {
        row.sort_unstable();
    }
    let mut result = 0;
    let m = grid.len();
    let n = grid[0].len();
    for j in 0..n {
        let mut max = 0;
        for i in 0..m {
            max = max.max(grid[i][j]);
        }
        result += max;
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 4], vec![3, 3, 1]]), 8);
        assert_eq!(func(vec![vec![10]]), 10);
    }
    test(delete_greatest_value);
}
