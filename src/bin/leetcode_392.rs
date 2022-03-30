//! 判断子序列


pub fn is_subsequence(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut ps = 0;
    let mut pt = 0;
    while ps < s.len() && pt < t.len() {
        if s[ps] == t[pt] {
            ps += 1;
            pt += 1;
        } else {
            pt += 1;
        }
    }
    return ps == s.len();
}

/// 大量时间用在t中寻找下一个匹配字符
/// 预处理，给出后面每一个字母的第一次出现位置
pub fn is_subsequence_dp(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let t_len = t.len();
    let mut dp = vec![vec![0; 26]; t_len + 1];
    // 构建
    for i in 0..26 {
        dp[t_len][i] = t_len;
    }
    for i in (0..t_len).rev() {
        for j in 0..26 {
            if t[i] == j + b'a' {
                dp[i][j as usize] = i;
            } else {
                dp[i][j as usize] = dp[i + 1][j as usize];
            }
        }
    }

    let mut p = 0;
    for i in 0..s.len() {
        if dp[p][(s[i] - b'a') as usize] == t_len { return false; }
        p = dp[p][(s[i] - b'a') as usize] + 1;
    }

    true
}


fn main() {
    fn test(func: fn(s: String, t: String) -> bool) {
        assert_eq!(func(String::from("abc"), String::from("ahbgdc")), true);
        assert_eq!(func(String::from("axc"), String::from("ahbgdc")), false);
    }
    test(is_subsequence);
    test(is_subsequence_dp);
}
