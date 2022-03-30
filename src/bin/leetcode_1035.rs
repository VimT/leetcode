//! 不相交的线

/// 计算nums1, nums2 的最长公共子序列的长度
/// dp[i][j] = nums1[0:i] and nums2[0:j] 的最长公共子序列的长度
/// dp[i][j] = dp[i-1][j-1] if nums1[i] == nums2[j] else {dp[i-1][j].max(dp[i][j-1])}
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums2.len() + 1];
    for c in nums1 {
        let mut pre = 0;
        for j in 1..=nums2.len() {
            let tmp = dp[j];
            if nums2[j - 1] == c {
                dp[j] = pre + 1;
            } else {
                dp[j] = dp[j].max(dp[j - 1]);
            }
            pre = tmp;
        }
    }
    dp[nums2.len()]
}

fn main() {
    assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
    assert_eq!(max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]), 3);
    assert_eq!(max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]), 2);
}
