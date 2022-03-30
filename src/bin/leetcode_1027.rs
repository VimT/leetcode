//! 最长等差数列

pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![vec![1; 1001]; len];
    let mut result = 0;
    for i in (0..len).rev() {
        for j in i + 1..len {
            let d = (nums[j] - nums[i] + 500) as usize;
            dp[i][d] = dp[i][d].max(dp[j][d] + 1);
            result = result.max(dp[i][d]);
        }
    }
    result
}

fn main() {
    assert_eq!(longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    assert_eq!(longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    assert_eq!(longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]), 4);
}
