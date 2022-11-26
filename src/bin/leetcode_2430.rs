//! 对字母串可执行的最大删除数

/// dp[i] 表示 删除s[i:]需要的最大操作数
pub fn delete_string(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut lcp = vec![vec![0; len + 1]; len + 1];  // s[i:] 和 s[j:] 的最长公共前缀
    for i in (0..len).rev() {
        for j in (0..len).rev() {
            if s[i] == s[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }
    let mut dp = vec![1; len];
    for i in (0..len).rev() {
        for j in 1..=(len - i) / 2 {
            if lcp[i][i + j] >= j {
                dp[i] = dp[i].max(dp[i + j] + 1);
            }
        }
    }
    dp[0]
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abcabcdabc")), 2);
        assert_eq!(func(String::from("aaabaab")), 4);
        assert_eq!(func(String::from("aaaaa")), 5);
    }
    test(delete_string);
}
