//! 和为目标值的最长子序列的长度

pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp = vec![i32::MIN; target + 1];
    dp[0] = 0;
    for num in nums {
        for i in (num as usize..=target).rev() {
            dp[i] = dp[i].max(dp[i - num as usize] + 1);
        }
    }
    dp[target].max(-1)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5], 9), 3);
        assert_eq!(func(vec![4, 1, 3, 2, 1, 5], 7), 4);
        assert_eq!(func(vec![1, 1, 5, 4, 5], 3), -1);
    }
    test(length_of_longest_subsequence);
}
