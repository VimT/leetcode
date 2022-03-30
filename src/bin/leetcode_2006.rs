//! 差的绝对值为 K 的数对数目

use std::collections::HashMap;

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut m = HashMap::new();
    let mut result = 0;
    for num in nums {
        if let Some(&v) = m.get(&(num + k)) {
            result += v;
        }
        if let Some(&v) = m.get(&(num - k)) {
            result += v;
        }
        *m.entry(num).or_insert(0i32) += 1;
    }
    result
}

fn main() {
    assert_eq!(count_k_difference(vec![1, 2, 2, 1], 1), 4);
    assert_eq!(count_k_difference(vec![1, 3], 3), 0);
    assert_eq!(count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
}
