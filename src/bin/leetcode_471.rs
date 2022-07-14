//! 编码最短长度的字符串

/// leetcode 459 找重复子字符串
pub fn encode(s: String) -> String {
    let len = s.len();
    let mut dp = vec![vec![String::new(); len]; len];
    for i in (0..len).rev() {
        for j in i..len {
            let tmp = &s[i..=j];
            if let Some(idx) = (format!("{}{}", &tmp, &tmp))[1..tmp.len() * 2 - 1].find(&tmp) {
                let encode = format!("{}[{}]", tmp.len() / (idx + 1), dp[i][i + idx]);
                if encode.len() < tmp.len() {
                    dp[i][j] = encode;
                }
            }
            if dp[i][j].is_empty() { dp[i][j] = tmp.to_string() }
            for k in i + 1..j {
                if dp[i][k].len() + dp[k + 1][j].len() < dp[i][j].len() {
                    dp[i][j] = dp[i][k].clone() + &dp[k + 1][j];
                }
            }
        }
    }
    dp[0][len - 1].clone()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("aabcaabcd")), String::from("2[aabc]d"));
        assert_eq!(func(String::from("aaa")), String::from("aaa"));
        assert_eq!(func(String::from("aaaaa")), String::from("5[a]"));
        assert_eq!(func(String::from("aaaaaaaaaa")), String::from("10[a]"));
    }
    test(encode);
}
