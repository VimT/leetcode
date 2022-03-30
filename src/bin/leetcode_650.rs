//! 650. 只有两个键的键盘

pub fn min_steps(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![usize::MAX; n + 1];
    dp[1] = 0;
    for i in 2..=n {
        let mut j = 1;
        while j * j <= i {
            if i % j == 0 {
                dp[i] = dp[i].min(dp[j] + i / j);
                dp[i] = dp[i].min(dp[i / j] + j);
            }
            j += 1;
        }
    }
    dp[n] as i32
}

/// 正则表达式匹配
fn main() {
    assert_eq!(min_steps(6), 5);
    assert_eq!(min_steps(7), 7);
    assert_eq!(min_steps(1), 0);
    assert_eq!(min_steps(3), 3);
    assert_eq!(min_steps(4), 4);
    assert_eq!(min_steps(5), 5);
}
