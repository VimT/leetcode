//! 两个字符串的最小ASCII删除和

pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let len1 = s1.len();
    let len2 = s2.len();
    let mut dp = vec![vec![i32::MAX; len2 + 1]; len1 + 1];
    dp[0][0] = 0;
    for i in 1..=len1 {
        dp[i][0] = dp[i - 1][0] + s1[i - 1] as i32;
    }
    for i in 1..=len2 {
        dp[0][i] = dp[0][i - 1] + s2[i - 1] as i32;
    }
    for i in 1..=len1 {
        for j in 1..=len2 {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] + s1[i - 1] as i32).min(dp[i][j - 1] + s2[j - 1] as i32);
            }
        }
    }
    dp[len1][len2]
}

fn main() {
    assert_eq!(minimum_delete_sum(String::from("sea"), String::from("eat")), 231);
    assert_eq!(minimum_delete_sum(String::from("delete"), String::from("leet")), 403);
}
