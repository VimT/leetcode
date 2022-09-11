//! 相等行列对

use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<Vec<i32>, i32> = HashMap::new();
    for row in &grid {
        *map.entry(row.clone()).or_default() += 1;
    }
    let n = grid.len();
    let mut result = 0;
    for i in 0..n {
        let col: Vec<i32> = grid.iter().map(|x| x[i]).collect();
        result += map.get(&col).cloned().unwrap_or(0);
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]), 1);
        assert_eq!(func(vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]]), 3);
    }
    test(equal_pairs);
}
