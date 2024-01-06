//! 三次操作后最大值与最小值的最小差

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 3 { return 0; }
    nums.sort_unstable();
    let mut result = i32::MAX;
    for left in 0..=3 {
        let right = 3 - left;
        result = result.min(nums[len - 1 - right] - nums[left])
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![5, 3, 2, 4]), 0);
        assert_eq!(func(vec![1, 5, 0, 10, 14]), 1);
        assert_eq!(func(vec![3, 100, 20]), 0);
    }
    test(min_difference);
}
