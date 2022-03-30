//! 最小差值 II

/// 先当成所有数都已经减过了，再从左边遍历，看前多少位要加2×k
pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = nums[len - 1] - nums[0];
    for (i, &num) in nums[..len - 1].iter().enumerate() {
        result = result.min((num + 2 * k).max(nums[len - 1]) - (nums[i + 1]).min(nums[0] + 2 * k));
    }
    result
}

fn main() {
    assert_eq!(smallest_range_ii(vec![1], 0), 0);
    assert_eq!(smallest_range_ii(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_ii(vec![1, 3, 6], 3), 3);
}
