//! 使数组和能被 P 整除

use std::collections::HashMap;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut cursum = 0;
    let len = nums.len() as i32;
    let mut result = len + 1;
    let sum = nums.iter().fold(0, |sum, &num| (sum + num) % p);
    if sum == 0 { return 0; }
    let mut map = HashMap::new();
    map.insert(0, 0);
    for (num, i) in nums.into_iter().zip(1..) {
        cursum = (cursum + num) % p;
        if let Some(&j) = map.get(&((cursum + p - sum) % p)) {
            result = result.min(i - j);
        }
        map.insert(cursum, i);
    }
    if result >= len { -1 } else { result }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, p: i32) -> i32) {
        assert_eq!(func(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(func(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(func(vec![1, 2, 3], 3), 0);
        assert_eq!(func(vec![1, 2, 3], 7), -1);
        assert_eq!(func(vec![1000000000, 1000000000, 1000000000], 3), 0);
    }
    test(min_subarray);
}
