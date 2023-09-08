//! 数组的最大美丽值

use std::collections::BTreeMap;

/// 差分数组
pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let mut diff: BTreeMap<i32, i32> = BTreeMap::new();
    for num in nums {
        *diff.entry(num).or_default() += 1;
        *diff.entry(num + k + k + 1).or_default() -= 1;
    }
    let mut cursum = 0;
    let mut result = 0;
    for (_, cnt) in diff {
        cursum += cnt;
        result = result.max(cursum);
    }
    result
}

/// 排序+双指针
pub fn maximum_beauty2(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut left = 0;
    let mut result = 0;
    for (right, &num) in nums.iter().enumerate() {
        while num - nums[left] > 2 * k {
            left += 1;
        }
        result = result.max(right - left + 1);
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![4, 6, 1, 2], 2), 3);
        assert_eq!(func(vec![1, 1, 1, 1], 10), 4);
    }
    test(maximum_beauty);
    test(maximum_beauty2);
}
