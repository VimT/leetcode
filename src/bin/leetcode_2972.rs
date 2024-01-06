//! 统计移除递增子数组的数目 II

/// 双指针
pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
    let mut i = 1;
    let len = nums.len();
    while i < len && nums[i - 1] < nums[i] {
        i += 1;
    }
    if i == len { return (len * (len + 1) / 2) as i64; }
    let mut result = i + 1;
    for j in (0..len).rev() {
        while i > 0 && nums[i - 1] >= nums[j] {
            i -= 1;
        }
        result += i + 1;
        if nums[j - 1] >= nums[j] { break; }
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![8, 7, 6, 6]), 3);
        assert_eq!(func(vec![1, 2, 3, 4]), 10);
        assert_eq!(func(vec![6, 5, 7, 8]), 7);
    }
    test(incremovable_subarray_count);
}
