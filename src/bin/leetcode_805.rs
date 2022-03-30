//! 数组的均值分割

use std::collections::HashSet;

/// sum(B) / K = sum(A) / N
/// 我们可以将 A 中的每个元素减去它们的平均值，这样 A 的平均值变为 0。此时我们的问题变成：找出若干个元素组成集合 B，这些元素的和为 0。
pub fn split_array_same_average(mut nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len == 1 { return false; }
    // 每个数扩大len倍，避免使用浮点数计算
    let mean = nums.iter().sum::<i32>();
    for num in &mut nums {
        *num = *num * len as i32 - mean;
    }
    let mut left: HashSet<i32> = HashSet::new();
    let half = len / 2;
    // 通过set遍历，而不是 0..1<<half，能一定程度上减枝
    for i in 0..half {
        let mut new_left = HashSet::with_capacity(left.len() * 2 + 1);
        new_left.insert(nums[i]);
        for num in left {
            new_left.insert(num);
            new_left.insert(num + nums[i]);
        }
        left = new_left;
    }
    if left.contains(&0) { return true; }
    let mut right: HashSet<i32> = HashSet::new();
    for j in half..len {
        let mut new_right = HashSet::with_capacity(right.len() * 2 + 1);
        new_right.insert(nums[j]);
        for num in right {
            new_right.insert(num);
            new_right.insert(num + nums[j]);
        }
        right = new_right;
    }
    if right.contains(&0) { return true; }
    let left_sum: i32 = nums[..half].iter().sum();
    for num in left {
        // 需要 num != left_sum，因为 [1,3] => [-2,2]
        if right.contains(&(-num)) && num != left_sum {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(split_array_same_average(vec![3, 1, 2]), true);
    assert_eq!(split_array_same_average(vec![2, 2]), true);
    assert_eq!(split_array_same_average(vec![3, 1]), false);
    assert_eq!(split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]), true);
}
