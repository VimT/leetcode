//! 完全平方数

/// dp[i] = min(dp[i], dp[i - j * j] + 1)
pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = i;
    }
    for i in 1..=n {
        let mut j = 1;
        while i >= j * j {
            dp[i] = dp[i].min(dp[i - j * j] + 1);
            j += 1;
        }
    }
    return dp[n] as i32;
}

fn main() {
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
