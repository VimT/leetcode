//! 新 21 点

/// dp[x] 表示从得分为 x 的情况开始游戏并且获胜的概率
pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
    if k == 0 { return 1_f64; }
    let n = n as usize;
    let k = k as usize;
    let w = w as usize;
    let mut dp = vec![0f64; k + w + 1];
    let mut i = k;
    while i <= n && i < k + w {
        dp[i] = 1_f64;
        i += 1;
    }
    dp[k - 1] = 1_f64 * (n - k + 1).min(w) as f64 / w as f64;
    for i in (0..k - 1).rev() {
        dp[i] = dp[i + 1] - (dp[i + w + 1] - dp[i + 1]) / w as f64;
    }
    dp[0]
}

fn main() {
    assert_eq!(new21_game(10, 1, 10), 1.00000);
    assert_eq!(new21_game(6, 1, 10), 0.60000);
    assert_eq!(new21_game(21, 17, 10), 0.7327777870686083);
}
