//! 对角线上不同值的数量差

use std::collections::HashSet;

pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut top_left = vec![vec![0; n]; m];
    let mut bottom_right = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            let mut set = HashSet::new();
            {
                let (mut i, mut j) = (i, j);
                while i > 0 && j > 0 {
                    set.insert(grid[i - 1][j - 1]);
                    i -= 1;
                    j -= 1;
                }
            }
            top_left[i][j] = set.len() as i32;

            {
                set.clear();
                let (mut i, mut j) = (i + 1, j + 1);
                while i < m && j < n {
                    set.insert(grid[i][j]);
                    i += 1;
                    j += 1;
                }
            }
            bottom_right[i][j] = set.len() as i32;
        }
    }

    for i in 0..m {
        for j in 0..n {
            top_left[i][j] = (top_left[i][j] - bottom_right[i][j]).abs();
        }
    }
    top_left
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]]), vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
        assert_eq!(func(vec![vec![1]]), vec![vec![0]]);
    }
    test(difference_of_distinct_values);
}
