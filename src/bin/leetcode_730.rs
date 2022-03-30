//! 统计不同回文子序列

/// 核心是怎么去重，引入一个状态量：x, 表示s[i]==s[j]==x
pub fn count_palindromic_subsequences(s: String) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![vec![0; len]; len]; 4];
    for i in (0..len).rev() {
        for j in i..len {
            for k in 0..4 {
                let c = b'a' + k as u8;
                if i == j {
                    dp[k][i][j] = if s[i] == c { 1 } else { 0 };
                } else {
                    if s[i] != c {
                        dp[k][i][j] = dp[k][i + 1][j];
                    } else if s[j] != c {
                        dp[k][i][j] = dp[k][i][j - 1]
                    } else {
                        if j == i + 1 {
                            dp[k][i][j] = 2;
                        } else {
                            dp[k][i][j] = 2;
                            for m in 0..4 {
                                dp[k][i][j] += dp[m][i + 1][j - 1];
                                dp[k][i][j] %= MOD;
                            }
                        }
                    }
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..4 {
        result += dp[i][0][len - 1];
        result %= MOD;
    }
    result
}

fn main() {
    assert_eq!(count_palindromic_subsequences(String::from("bccb")), 6);
    assert_eq!(count_palindromic_subsequences(String::from("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba")), 104860361);
}
