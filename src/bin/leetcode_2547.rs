//! 拆分数组的最小代价

/// dp[i] 表示前i个拆分的最小代价
pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut dp = vec![i32::MAX / 2; len + 1];
    dp[0] = 0;
    for i in 0..len {
        let mut cnt = vec![0; len];
        let mut multi = 0;
        for j in (0..=i).rev() {
            cnt[nums[j] as usize] += 1;
            let c = cnt[nums[j] as usize];
            if c == 2 { multi += 2; } else if c > 2 { multi += 1; }
            dp[i + 1] = dp[i + 1].min(dp[j] + k + multi);
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
        assert_eq!(func(vec![1, 2, 1, 2, 1], 2), 6);
        assert_eq!(func(vec![1, 2, 1, 2, 1], 5), 10);
    }
    test(min_cost);
}
