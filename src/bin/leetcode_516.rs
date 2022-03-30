//! 最长回文子序列

pub fn longest_palindrome_subseq(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![0; len]; len];
    for i in 0..len {
        dp[i][i] = 1;
    }
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            dp[i][j] = if s[i] == s[j] {
                2 + dp[i + 1][j - 1]
            } else {
                dp[i + 1][j].max(dp[i][j - 1])
            }
        }
    }
    dp[0][len - 1]
}

fn main() {
    assert_eq!(longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(longest_palindrome_subseq("cbbd".to_string()), 2);
    assert_eq!(longest_palindrome_subseq("abccba".to_string()), 6);
    assert_eq!(longest_palindrome_subseq("abcgcba".to_string()), 7);
    assert_eq!(longest_palindrome_subseq("aeebcgcbffa".to_string()), 7);
}

