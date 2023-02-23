//! 最少得分子序列

/// 转换成：前缀匹配一部分，后缀匹配一部分，看前缀后缀最多能匹配多少，匹配不了的就删掉
pub fn minimum_score(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let slen = s.len();
    let tlen = t.len();
    let mut i = slen;
    let mut right = vec![-1; tlen + 1];
    right[tlen] = slen as i32;
    for j in (0..tlen).rev() {
        while i > 0 && s[i - 1] != t[j] {
            i -= 1;
        }
        right[j] = i as i32 - 1;
        if i > 0 {
            i -= 1;
        }
    }
    let mut left = vec![-1; tlen + 1];
    i = 0;
    for j in 0..tlen {
        while i < slen && s[i] != t[j] {
            i += 1;
        }
        left[j + 1] = i as i32;
        if i < slen { i += 1; }
    }
    let mut l = 0;
    let mut result = i32::MAX;
    for r in 0..=tlen {
        while left[l] < right[r] {
            l += 1;
            result = result.min((r as i32 - l as i32 + 1).max(0));
            if result == 0 { return 0; }
        }
    }
    result
}


/// 上一个是看t的前缀能匹配的s的哪里，其实用s的前缀看能匹配到t的哪里更方便
pub fn minimum_score2(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut j = 0;
    let mut dp = vec![0; s.len() + 1]; // dp[i]表示 s[..i] 匹配了 t[..dp[i]]
    for i in 0..s.len() {
        if j < t.len() && s[i] == t[j] {
            j += 1;
        }
        dp[i + 1] = j;
    }
    let mut result = t.len() - dp[s.len()];
    j = t.len();  // 表示 s[i..] 匹配了 t[j..]
    for i in (0..s.len()).rev() {
        if j > 0 && s[i] == t[j - 1] {
            j -= 1;
        }
        result = result.min(j - dp[i]); // 如果j-dp[i]为负，自动溢出成大值，所以不需要判断
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, t: String) -> i32) {
        assert_eq!(func(String::from("acdedcdbabecdbebda"), String::from("bbecddb")), 1);
        assert_eq!(func(String::from("gbjbacdiiiecgceeafdcdhjhhcjfchjbejibhejgjhhhjhifahfbbcfcajehjgcjgffjdejbhjahgffgdaifhhgaadjiabfdf"), String::from("hidefgbgjghceagdaaib")), 5);
        assert_eq!(func(String::from("abacaba"), String::from("bzaa")), 1);
        assert_eq!(func(String::from("cde"), String::from("xyz")), 3);
    }
    test(minimum_score);
    test(minimum_score2);
}
