//! 拼接数组的最大分数

pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn cal(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let len = nums2.len();
        let mut dp = vec![[0, 0, 0]; len + 1];
        for i in 1..=len {
            // 没有换
            dp[i][0] = dp[i - 1][0] + nums1[i - 1];
            // 正在换
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][0]) + nums2[i - 1];
            // 换结束
            dp[i][2] = dp[i - 1][1].max(dp[i - 1][2]) + nums1[i - 1];
        }
        dp[len][0].max(dp[len][1]).max(dp[len][2])
    }
    cal(&nums1, &nums2).max(cal(&nums2, &nums1))
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![60, 60, 60], vec![10, 90, 10]), 210);
        assert_eq!(func(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]), 220);
        assert_eq!(func(vec![7, 11, 13], vec![1, 1, 1]), 31);
    }
    test(maximums_spliced_array);
}
