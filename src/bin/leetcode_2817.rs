//! 限制条件下元素之间的最小绝对差

use std::collections::BTreeSet;

pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
    let mut m: BTreeSet<i32> = BTreeSet::new();
    let mut result = i32::MAX;
    for (&v, &y) in nums.iter().zip(&nums[x as usize..]) { // 用zip遍历双指针
        m.insert(v);
        if let Some(&next) = m.range(y..).next() {
            result = result.min(next - y);
        }
        if let Some(&prev) = m.range(..y).next_back() {
            result = result.min(y - prev);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, x: i32) -> i32) {
        assert_eq!(func(vec![5, 3, 2, 10, 15], 1), 1);
        assert_eq!(func(vec![4, 3, 2, 4], 2), 0);
        assert_eq!(func(vec![1, 2, 3, 4], 3), 3);
    }
    test(min_absolute_difference);
}
