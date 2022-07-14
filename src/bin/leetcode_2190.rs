//! 数组中紧跟 key 之后出现最频繁的数字

use std::collections::HashMap;

pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    let len = nums.len();
    let mut cnt = HashMap::new();
    for i in 0..len - 1 {
        if nums[i] == key {
            *cnt.entry(nums[i + 1]).or_insert(0i32) += 1;
        }
    }
    *cnt.iter().max_by_key(|x| x.1).unwrap().0
}

fn main() {
    assert_eq!(most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
    assert_eq!(most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
}
