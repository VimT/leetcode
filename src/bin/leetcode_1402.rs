//! 做菜顺序

pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    satisfaction.sort_unstable();
    let len = satisfaction.len();
    let mut dp = vec![i32::MIN / 2; len + 1]; // 前i道菜，做了j个最大喜爱时间
    dp[0] = 0;
    for i in 0..len {
        for j in (1..=i + 1).rev() {
            dp[j] = dp[j].max(dp[j - 1] + j as i32 * satisfaction[i]);
        }
    }
    0.max(*dp.iter().max().unwrap())
}

fn main() {
    fn test(func: fn(satisfaction: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-1, -8, 0, 5, -9]), 14);
        assert_eq!(func(vec![4, 3, 2]), 20);
        assert_eq!(func(vec![-1, -4, -5]), 0);
    }
    test(max_satisfaction);
}
