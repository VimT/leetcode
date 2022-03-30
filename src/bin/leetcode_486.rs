//! 预测赢家

pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    fn dfs(nums: &Vec<i32>, left: usize, right: usize, diff: i32) -> bool {
        let turn = nums.len() - (right - left + 1);
        return if turn & 1 == 1 {
            if left == right { return diff - nums[left] >= 0; }
            if (left < nums.len() && !dfs(nums, left + 1, right, diff - nums[left])) || (right > 0 && !dfs(nums, left, right - 1, diff - nums[right])) {
                return false;
            }
            true
        } else {
            if left == right { return diff + nums[left] >= 0; }
            if (left < nums.len() && dfs(nums, left + 1, right, diff + nums[left])) || (right > 0 && dfs(nums, left, right - 1, diff + nums[right])) {
                return true;
            }
            false
        };
    }
    dfs(&nums, 0, nums.len() - 1, 0)
}

/// dp[i][j]：作为先手，在区间 nums[i..j] 里进行选择可以获得的相对分数
pub fn predict_the_winner_dp(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut dp = vec![vec![i32::MIN; len]; len];
    for i in 0..len {
        dp[i][i] = nums[i];
    }
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
        }
    }
    dp[0][len - 1] >= 0
}

fn main() {
    assert_eq!(predict_the_winner_dp(vec![0]), true);
    assert_eq!(predict_the_winner_dp(vec![1, 5, 2]), false);
    assert_eq!(predict_the_winner_dp(vec![1, 5, 233, 7]), true);
}
