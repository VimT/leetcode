//! 两个字符串的删除操作

pub fn min_distance(word1: String, word2: String) -> i32 {
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let len1 = w1.len();
    let len2 = w2.len();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..len1 {
        for j in 0..len2 {
            dp[i + 1][j + 1] = (dp[i + 1][j]).max(dp[i][j + 1]);
            if w1[i] == w2[j] { dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1); }
        }
    }
    (len1 + len2 - 2 * dp[len1][len2]) as i32
}

fn main() {
    assert_eq!(min_distance(String::from("sea"), String::from("eat")), 2);
    assert_eq!(min_distance(String::from("leetcode"), String::from("etco")), 4);
}
