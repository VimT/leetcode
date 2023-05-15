//! 最大或值


/// dp[i][j] 表示前i项，已经使用了k次左移的最大值
pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let k = k as usize;
    let mut dp = vec![vec![0; k + 1]; len + 1];
    for i in 0..len {
        let num = nums[i] as i64;
        for k in 0..=k {
            for pk in 0..=k {
                dp[i + 1][k] = dp[i + 1][k].max(dp[i][pk] | (num << (k - pk)));
            }
        }
    }
    *dp[len].iter().max().unwrap()
}

/// 贪心，一个数会用完所有左移次数
pub fn maximum_or2(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut suf = vec![0; len + 1];
    for i in (0..len).rev() {
        suf[i] = suf[i + 1] | nums[i];
    }
    let mut cur = 0;
    let mut result = 0;
    for i in 0..len {
        result = result.max(cur | (nums[i] as i64) << k | suf[i + 1] as i64);
        cur |= nums[i] as i64;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![12, 9], 1), 30);
        assert_eq!(func(vec![8, 1, 2], 2), 35);
    }
    test(maximum_or);
    test(maximum_or2);
}
