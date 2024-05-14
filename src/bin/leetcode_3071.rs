//! 在矩阵上写出字母 Y 所需的最少操作次数

pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mid = n / 2;
    let mut result = i32::MAX;
    for y in 0..=2 {
        for other in 0..=2 {
            if y == other { continue; }
            let mut cnt = 0;
            for i in 0..n {
                for j in 0..n {
                    if (i == j && i <= mid) || (i + j == n - 1 && j >= mid) || (i >= mid && j == mid) {
                        if grid[i][j] != y {
                            cnt += 1;
                        }
                    } else {
                        if grid[i][j] != other {
                            cnt += 1;
                        }
                    }
                }
            }
            result = result.min(cnt);
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]]), 3);
        assert_eq!(func(vec![vec![0, 1, 0, 1, 0], vec![2, 1, 0, 1, 2], vec![2, 2, 2, 0, 1], vec![2, 2, 2, 2, 2], vec![2, 1, 2, 2, 2]]), 12);
    }
    test(minimum_operations_to_write_y);
}
