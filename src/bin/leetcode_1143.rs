//! 最长公共子序列

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let w1 = text1.as_bytes();
    let w2 = text2.as_bytes();
    let len1 = w1.len();
    let len2 = w2.len();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..len1 {
        for j in 0..len2 {
            dp[i + 1][j + 1] = (dp[i + 1][j]).max(dp[i][j + 1]);
            if w1[i] == w2[j] { dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1); }
        }
    }
    dp[len1][len2]
}

fn main() {
    assert_eq!(longest_common_subsequence(String::from("abcde"), String::from("ace")), 3);
    assert_eq!(longest_common_subsequence(String::from("abc"), String::from("abc")), 3);
    assert_eq!(longest_common_subsequence(String::from("abc"), String::from("def")), 0);
}
