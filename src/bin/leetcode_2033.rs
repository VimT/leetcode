//! 获取单值网格的最小操作数

pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut sum = 0;
    let mut min = grid[0][0];
    let mut max = grid[0][0];
    for i in 0..m {
        for j in 0..n {
            sum += grid[i][j];
            min = min.min(grid[i][j]);
            max = max.max(grid[i][j]);
        }
    }
    let mut start = min;
    let bei = x * (m * n) as i32;
    for i in min..=min + x {
        if (sum as i64 - i as i64 * bei as i64) % x as i64 == 0 {
            start = i;
            break;
        }
    }
    for i in 0..m {
        for j in 0..n {
            if (grid[i][j] - start) % x != 0 {
                return -1;
            }
        }
    }
    let mut left = 0;
    let mut right = (max - start) / x;
    let mut result = i32::MAX;
    while left <= right {
        let mid = left + (right - left) / 2;
        let target1 = start + mid * x;
        let target2 = start + (mid + 1) * x;
        let mut num1 = 0;
        let mut num2 = 0;
        for i in 0..m {
            for j in 0..n {
                num1 += (grid[i][j] - target1).abs() / x;
                num2 += (grid[i][j] - target2).abs() / x;
            }
        }
        result = result.min(num1).min(num2);
        if num1 > num2 {
            left = mid + 1;
        } else {
            right = mid;
        }
        if left == right {
            break;
        }
    }
    result
}

fn main() {
    assert_eq!(min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
    assert_eq!(min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
    assert_eq!(min_operations(vec![vec![1, 2], vec![3, 4]], 2), -1);
}
