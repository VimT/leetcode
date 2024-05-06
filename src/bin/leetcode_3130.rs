//! 找出所有稳定的二进制数组 II


pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let zero = zero as usize;
    let one = one as usize;
    let limit = limit as usize;
    let mut dp = vec![vec![[0i32; 2]; one + 1]; zero + 1]; // dp[i][j][k] 表示zero=i, one=j时，且以k结尾的符合要求的数量
    for i in 0..=zero.min(limit) {
        dp[i][0][0] = 1;
    }
    for j in 0..=one.min(limit) {
        dp[0][j][1] = 1;
    }
    for i in 1..=zero {
        for j in 1..=one {
            dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1] - if i > limit { dp[i - limit - 1][j][1] } else { 0 }).rem_euclid(MOD);
            dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1] - if j > limit { dp[i][j - limit - 1][0] } else { 0 }).rem_euclid(MOD);
        }
    }
    (dp[zero][one][0] + dp[zero][one][1]).rem_euclid(MOD)
}


fn main() {
    fn test(func: fn(zero: i32, one: i32, limit: i32) -> i32) {
        assert_eq!(func(1, 4, 2), 1);
        assert_eq!(func(1, 4, 1), 0);
        assert_eq!(func(1, 3, 1), 0);
        assert_eq!(func(1, 1, 2), 2);
        assert_eq!(func(1, 2, 1), 1);
        assert_eq!(func(3, 3, 2), 14);
    }
    test(number_of_stable_arrays);
}
