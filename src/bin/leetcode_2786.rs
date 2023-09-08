//! 访问数组中的位置使分数最大

pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    let len = nums.len();
    let x = x as i64;
    let mut dp = [i64::MIN / 2; 2];
    dp[(nums[0] & 1) as usize] = nums[0] as i64;
    for i in 1..len {
        let mut new_dp = dp.clone();
        new_dp[(nums[i] & 1) as usize] = new_dp[(nums[i] & 1) as usize]
            .max(dp[0] + nums[i] as i64 - if nums[i] & 1 == 1 { x } else { 0 })
            .max(dp[1] + nums[i] as i64 - if nums[i] & 1 == 0 { x } else { 0 });
        dp = new_dp;
    }
    dp[0].max(dp[1])
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, x: i32) -> i64) {
        assert_eq!(func(vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79], 74), 470);
        assert_eq!(func(vec![2, 3, 6, 1, 9, 2], 5), 13);
        assert_eq!(func(vec![2, 4, 6, 8], 3), 20);
    }
    test(max_score);
}
