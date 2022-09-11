//! 检查数组是否存在有效划分

pub fn valid_partition(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut dp = vec![false; len + 1];
    dp[0] = true;
    for i in 1..len {
        if i > 0 && nums[i] == nums[i - 1] {
            dp[i + 1] |= dp[i - 1];
        }
        if i > 1 && ((nums[i] == nums[i - 1] && nums[i] == nums[i - 2]) || (nums[i] == nums[i - 1] + 1 && nums[i] == nums[i - 2] + 2)) {
            dp[i + 1] |= dp[i - 2];
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![4, 4, 4, 5, 6]), true);
        assert_eq!(func(vec![1, 1, 1, 2]), false);
    }
    test(valid_partition);
}
