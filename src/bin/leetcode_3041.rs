//! 修改数组后最大化数组中的连续元素数目

pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![[1, 1]; len];
    nums.sort_unstable();
    let mut result = 1;
    for i in 1..len {
        if nums[i] - nums[i - 1] == 1 {
            dp[i][0] = dp[i][0].max(dp[i - 1][0] + 1);
            dp[i][1] = dp[i][1].max(dp[i - 1][1] + 1);
        } else if nums[i] - nums[i - 1] == 2 {
            dp[i][0] = dp[i][0].max(dp[i - 1][1] + 1);
            dp[i][1] = 1;
        } else if nums[i] - nums[i - 1] == 0 {
            dp[i][0] = dp[i][0].max(dp[i - 1][0]);
            dp[i][1] = dp[i][1].max(dp[i - 1][1]).max(dp[i - 1][0] + 1);
        } else {
            dp[i][0] = 1;
            dp[i][1] = 1;
        }
        result = result.max(dp[i][0]).max(dp[i][1]);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, 5, 1, 1]), 3);
        assert_eq!(func(vec![1, 4, 7, 10]), 1);
    }
    test(max_selected_elements);
}
