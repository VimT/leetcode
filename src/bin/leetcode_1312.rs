//! 让字符串成为回文串的最少插入次数

/// dp[i][j] 表示将[i,j]变为回文串的最小操作次数
pub fn min_insertions(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![0; len]; len];
    for range in 2..=len {
        for i in 0..=len - range {
            let j = i + range - 1;
            if s[i] == s[j] {
                dp[i][j] = dp[i + 1][j - 1];
            } else {
                dp[i][j] = (dp[i + 1][j] + 1).min(dp[i][j - 1] + 1);
            }
        }
    }
    dp[0][len - 1]
}

/// 不用区间dp
pub fn min_insertions2(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![0; len]; len];
    for i in (0..len).rev() {
        for j in i + 1..len {
            if s[i] == s[j] {
                dp[i][j] = dp[i + 1][j - 1];
            } else {
                dp[i][j] = (dp[i + 1][j] + 1).min(dp[i][j - 1] + 1);
            }
        }
    }
    dp[0][len - 1]
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("zjveiiwvc")), 5);
        assert_eq!(func(String::from("zzazz")), 0);
        assert_eq!(func(String::from("mbadm")), 2);
        assert_eq!(func(String::from("leetcode")), 5);
    }
    test(min_insertions);
    test(min_insertions2);
}
