//! 正整数和负整数的最大计数

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let idx = nums.binary_search(&0).unwrap_or_else(|x| x);
    let mut left = idx + 1;
    while left > 0 && nums[left - 1] >= 0 {
        left -= 1;
    }
    let mut right = idx;
    while right < nums.len() && nums[right] <= 0 {
        right += 1;
    }
    left.max(nums.len() - right) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-2, -1, -1, 1, 2, 3]), 3);
        assert_eq!(func(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
        assert_eq!(func(vec![5, 20, 66, 1314]), 4);
    }
    test(maximum_count);
}
