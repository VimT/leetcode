//! 使数组和小于等于 x 的最少时间

/// 2900+分的题，很难想到是个DP
pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
    let len = nums2.len();
    let sum1: i32 = nums1.iter().sum();
    let sum2: i32 = nums2.iter().sum();
    let mut num: Vec<(i32, i32)> = nums2.into_iter().zip(nums1).collect();
    num.sort_unstable();
    let mut dp = vec![0; len + 1];  // dp[i][j] 表示前i个数，操作j次的最大减量
    for (x, y) in num {
        for i in (0..len).rev() {
            dp[i + 1] = dp[i + 1].max(dp[i] + (i as i32 + 1) * x + y);
        }
    }
    for i in 0..=len {
        if sum2 * i as i32 + sum1 - dp[i] <= x {
            return i as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32) {
        assert_eq!(func(vec![1, 7, 6, 2, 9], vec![4, 2, 3, 3, 0], 23), 4);
        assert_eq!(func(vec![7, 10, 1, 3, 7, 3, 2], vec![1, 1, 3, 0, 2, 2, 3], 22), 7);
        assert_eq!(func(vec![9, 2, 8, 3, 1, 9, 7, 6], vec![0, 3, 4, 1, 3, 4, 2, 1], 40), 8);
        assert_eq!(func(vec![4, 5, 3, 2, 3, 9, 5, 7, 10, 4], vec![4, 4, 0, 4, 1, 2, 4, 0, 4, 0], 47), -1);
        assert_eq!(func(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
        assert_eq!(func(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
    }
    test(minimum_time);
}
