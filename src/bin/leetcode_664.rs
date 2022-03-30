//! 奇怪的打印机

/// 令f[i][j] 表示打印完成区间 [i,j] 的最少操作数。
pub fn strange_printer(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![i32::MAX; len]; len + 1];
    for i in 0..len {
        dp[i][i] = 1;
    }
    for i in (0..len).rev() {
        for j in i + 1..len {
            if s[i] == s[j] {
                dp[i][j] = dp[i][j - 1];
            } else {
                for k in i..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j]);
                }
            }
        }
    }
    dp[0][len - 1]
}

fn main() {
    assert_eq!(strange_printer(String::from("abcacb")), 4);
    assert_eq!(strange_printer(String::from("aaabbb")), 2);
    assert_eq!(strange_printer(String::from("abc")), 3);
    assert_eq!(strange_printer(String::from("aba")), 2);
}
