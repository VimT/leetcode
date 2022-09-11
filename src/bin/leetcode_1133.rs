//! 最大唯一数

use std::collections::HashMap;

pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *cnt.entry(num).or_default() += 1;
    }
    let mut result = -1;
    for (k, v) in cnt {
        if v == 1 { result = result.max(k) }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![5, 7, 3, 9, 4, 9, 8, 3, 1]), 8);
        assert_eq!(func(vec![9, 9, 8, 8]), -1);
    }
    test(largest_unique_number);
}
