//! 缺失的第一个正数


use std::collections::HashSet;

pub fn first_missing_positive_set(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut ans = 1;
    loop {
        if set.contains(&ans) {
            ans += 1;
        } else {
            return ans;
        }
    }
}

/// 空间复杂度O(1)
/// f(nums[i]) = nums[i] - 1
/// nums[i] 应该在 nums[i] - 1 的位置上
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    for i in 0..len {
        while nums[i as usize] > 0 && nums[i as usize] <= len && nums[(nums[i as usize] - 1) as usize] != nums[i as usize] {
            let should_be = (nums[i as usize] - 1) as usize;
            nums.swap(should_be, i as usize);
        }
    }
    for i in 0..len {
        if nums[i as usize] != i + 1 { return i + 1; }
    }
    len + 1
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 0]), 3);
        assert_eq!(func(vec![3, 4, -1, 1]), 2);
        assert_eq!(func(vec![7, 8, 9, 11, 12]), 1);
    }
    test(first_missing_positive);
    test(first_missing_positive_set);
}
