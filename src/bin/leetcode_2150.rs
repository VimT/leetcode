//! 找出数组中的所有孤独数字

use std::collections::HashMap;

pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashMap::new();
    for &num in &nums {
        *set.entry(num).or_insert(0i32) += 1;
    }
    let mut result = vec![];
    for num in nums {
        if set[&num] == 1 && !set.contains_key(&(num - 1)) && !set.contains_key(&(num + 1)) {
            result.push(num);
        }
    }
    result
}

fn main() {
    assert_eq!(find_lonely(vec![10, 6, 5, 8]), [10, 8]);
    assert_eq!(find_lonely(vec![1, 3, 5, 3]), [1, 5]);
}
