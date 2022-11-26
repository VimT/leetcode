//! 不重叠回文子字符串的最大数目

/// O(n2) 算出每个字串是不是回文串，再O(n2) dp算出最大不重叠回文子串数
pub fn max_palindromes(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![false; len]; len];

    for l in 0..len {
        for i in 0..len - l {
            dp[i][i + l] = s[i] == s[i + l] && (l < 3 || dp[i + 1][i + l - 1]);
        }
    }

    let mut dp2 = vec![0; len];
    for i in k as usize - 1..len {
        if i > 0 { dp2[i] = dp2[i - 1]; }
        for j in 0..=i + 1 - k as usize {
            if dp[j][i] {
                dp2[i] = dp2[i].max(if j > 0 { dp2[j - 1] } else { 0 } + 1);
            }
        }
    }

    dp2[len - 1]
}

/// 直接DP+中心扩展法
pub fn max_palindromes2(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![0; len + 1];
    // 中心扩展法非常巧妙的遍历，这样可以同时兼容 奇数回文串和偶数回文串的情况
    for i in 0..2 * len - 1 {
        let mut l = i / 2;
        let mut r = l + (i & 1);
        dp[l + 1] = dp[l].max(dp[l + 1]);
        while r < len && s[l] == s[r] {
            if r + 1 - l >= k as usize {
                dp[r + 1] = dp[r + 1].max(dp[l] + 1);
            }
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("iqqibcecvrbxxj"), 1), 14);
        assert_eq!(func(String::from("abaccdbbd"), 3), 2);
        assert_eq!(func(String::from("adbcda"), 2), 0);
    }
    test(max_palindromes);
    test(max_palindromes2);
}
