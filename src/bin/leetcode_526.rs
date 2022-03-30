//! 优美的排列

/// 当我们想要计算dp[mask] 时，我们只需要在前 1count(mask) - 1 位都已经放置了数的情况下，
/// 考虑第 1count(mask) 位要放置的数即可，我们枚举当前位的符合条件的数，
/// 并将方案数累加到 dp[mask] 中即可。
pub fn count_arrangement(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;
    for mask in 1usize..1 << n {
        let num = mask.count_ones() as usize;
        for i in 0..n {
            if mask & (1 << i) > 0 && (num % (i + 1) == 0 || (i + 1) % num == 0) {
                dp[mask] += dp[mask ^ (1 << i)];
            }
        }
    }
    dp[(1 << n) - 1]
}

fn main() {
    assert_eq!(count_arrangement(2), 2);
    assert_eq!(count_arrangement(1), 1);
}
