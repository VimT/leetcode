//! 两个子序列的最大点积

pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let m = nums1.len();
    let n = nums2.len();
    let mut dp = vec![vec![i32::MIN / 2; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]).max(dp[i][j].max(0) + nums1[i] * nums2[j]);
        }
    }
    dp[m][n]
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, -2, 5], vec![3, 0, -6]), 18);
        assert_eq!(func(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(func(vec![-1, -1], vec![1, 1]), -1);
    }
    test(max_dot_product);
}
