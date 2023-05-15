//! 找出不同元素数目差数组

use std::collections::HashSet;

pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let mut s = HashSet::new();
    let len = nums.len();
    let mut suffix = vec![0; len + 1];
    for i in (0..len).rev() {
        s.insert(nums[i]);
        suffix[i] = s.len();
    }
    s.clear();
    let mut result = vec![0; len];
    for i in 0..len {
        s.insert(nums[i]);
        result[i] = s.len() as i32 - suffix[i + 1] as i32;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 3, 4, 5]), vec![-3, -1, 1, 3, 5]);
        assert_eq!(func(vec![3, 2, 3, 4, 2]), vec![-2, -1, 0, 2, 3]);
    }
    test(distinct_difference_array);
}
