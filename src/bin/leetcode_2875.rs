//! 无限数组的最短子数组

/// 滑动窗口
pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut target = target as i64;
    let mut result = usize::MAX;
    let multi = target / sum;
    target %= sum;
    sum = 0;
    let mut left = 0;
    for right in 0..len * 2 {
        sum += nums[right % len] as i64;
        while sum > target {
            sum -= nums[left % len] as i64;
            left += 1;
        }
        if sum == target {
            result = result.min(right + 1 - left);
        }
    }

    if result == usize::MAX { -1 } else { len as i32 * multi as i32 + result as i32 }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 6, 5, 5, 1, 1, 2, 5, 3, 1, 5, 3, 2, 4, 6, 6], 56), 16);
        assert_eq!(func(vec![1, 2, 2, 2, 1, 2, 1, 2, 1, 2, 1], 83), 53);
        assert_eq!(func(vec![1, 2, 3], 5), 2);
        assert_eq!(func(vec![1, 1, 1, 2, 3], 4), 2);
        assert_eq!(func(vec![2, 4, 6, 8], 3), -1);
    }
    test(min_size_subarray);
}
