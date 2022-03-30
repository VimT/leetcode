//! 组合总和 Ⅳ

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp = vec![0; target + 1];
    dp[0] = 1;
    for i in 1..=target {
        for &num in &nums {
            if num as usize <= i {
                dp[i] += dp[i - num as usize];
            }
        }
    }
    return dp[target];
}

fn main() {
    assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
    assert_eq!(combination_sum4(vec![9], 3), 0);
}
