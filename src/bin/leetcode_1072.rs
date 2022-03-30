//! 按列翻转得到最大值等行数

use std::collections::HashMap;

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut cnt_map = HashMap::new();
    for i in 0..m {
        let mut rev = matrix[i].clone();
        for j in 0..n {
            rev[j] = if rev[j] == 0 { 1 } else { 0 };
        }
        *cnt_map.entry(matrix[i].clone()).or_insert(0i32) += 1;
        *cnt_map.entry(rev).or_insert(0i32) += 1;
    }
    *cnt_map.values().max().unwrap()
}

fn main() {
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);
    assert_eq!(max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]), 2);
}
