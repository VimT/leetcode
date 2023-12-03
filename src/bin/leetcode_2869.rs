//! 收集元素的最少操作次数

use std::collections::HashSet;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut set: HashSet<i32> = (1..=k).collect();
    let len = nums.len();
    for (num, i) in nums.into_iter().rev().zip(1..) {
        if num <= k {
            set.remove(&num);
            if set.is_empty() {
                return i;
            }
        }
    }
    len as i32
}

/// 位运算
pub fn min_operations2(nums: Vec<i32>, k: i32) -> i32 {
    let mut set = (1i64 << k) - 1;
    for (num, i) in nums.into_iter().rev().zip(1..) {
        set &= !(1 << num - 1);
        if set == 0 {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3, 1, 5, 4, 2], 2), 4);
        assert_eq!(func(vec![3, 1, 5, 4, 2], 5), 5);
        assert_eq!(func(vec![3, 2, 5, 3, 1], 3), 4);
    }
    test(min_operations);
    test(min_operations2);
}
