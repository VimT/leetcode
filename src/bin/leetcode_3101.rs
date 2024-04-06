//! 交替子数组计数

pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    let mut left = 0;
    for right in 0..len {
        if right > 0 && nums[right] == nums[right - 1] {
            left = right;
        }
        result += (right - left + 1) as i64;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![0, 1, 1, 1]), 5);
        assert_eq!(func(vec![1, 0, 1, 0]), 10);
    }
    test(count_alternating_subarrays);
}
