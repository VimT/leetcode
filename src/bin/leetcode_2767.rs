//! 将字符串分割为最少的美丽子字符串

pub fn minimum_beautiful_substrings(s: String) -> i32 {
    fn dfs(s: &Vec<u8>, i: usize) -> Option<i32> {
        if i == s.len() { return Some(0); }
        if s[i] == b'0' {
            return None;
        }
        let mut cur = 0;
        let mut result = None;
        for j in i..s.len() {
            cur = cur * 2 + (s[j] - b'0') as i32;
            if matches!(cur, 1|5|25|125|625|3125|15625|78125|390625|1953125) {
                if let Some(sub) = dfs(s, j + 1) {
                    result = result.map(|x: i32| x.min(sub + 1)).or(Some(sub + 1));
                }
            }
        }
        result
    }
    let s = s.into_bytes();
    dfs(&s, 0).unwrap_or(-1)
}

/// dp[i] 表示把 [i..] 分割成美丽子字符串的最少次数
pub fn minimum_beautiful_substrings2(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![len as i32 + 1; len + 1];
    dp[len] = 0;
    for i in (0..len).rev() {
        if s[i] == b'0' { continue; }
        let mut cur = 0;
        for j in i..s.len() {
            cur = cur * 2 + (s[j] - b'0') as i32;
            if matches!(cur, 1|5|25|125|625|3125|15625|78125|390625|1953125) {
                dp[i] = dp[i].min(dp[j + 1] + 1);
            }
        }
    }
    if dp[0] > len as i32 { -1 } else { dp[0] }
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("1011")), 2);
        assert_eq!(func(String::from("111")), 3);
        assert_eq!(func(String::from("0")), -1);
    }
    test(minimum_beautiful_substrings);
    test(minimum_beautiful_substrings2);
}
