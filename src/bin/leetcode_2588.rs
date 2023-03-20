//! 统计美丽子数组数目

use std::collections::HashMap;

pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
    let mut cnt = HashMap::new();
    cnt.insert(0, 1);
    let mut result = 0;
    let mut cur = 0;
    for num in nums {
        cur ^= num;
        let cc = cnt.entry(cur).or_default();
        result += *cc;
        *cc += 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![4, 3, 1, 2, 4]), 2);
        assert_eq!(func(vec![1, 10, 4]), 0);
    }
    test(beautiful_subarrays);
}
