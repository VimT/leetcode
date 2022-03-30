//! 最长定差子序列

use std::collections::HashMap;

pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut m = HashMap::new();
    let len = arr.len();
    let mut result = 1;
    for i in 0..len {
        let target = arr[i] - difference;
        let cur = m.get(&target).unwrap_or(&0) + 1;
        result = result.max(cur);
        m.insert(arr[i], cur);
    }
    result
}

fn main() {
    assert_eq!(longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 0), 1);
    assert_eq!(longest_subsequence(vec![1, 1, 2, 2], 0), 2);
    assert_eq!(longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
}
