//!  旋转函数

pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![0; len];
    let mut sum = 0;
    for i in 0..len {
        sum += nums[i];
        dp[0] += nums[i] * i as i32;
    }
    let mut result = dp[0];
    for i in 1..len {
        dp[i] = dp[i - 1] + sum - len as i32 * nums[len - i];
        result = result.max(dp[i]);
    }
    result
}

fn main() {
    assert_eq!(max_rotate_function(vec![4, 3, 2, 6]), 26);
}
