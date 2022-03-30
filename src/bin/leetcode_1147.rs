//! 段式回文

/// 先kmp计算出next数组，再根据next数组计算dp
/// case： aaa, next[0][2] = 2，所以针对s[i] == s[j] 的情况，算dp[i+1][j-1]
pub fn longest_decomposition(text: String) -> i32 {
    let s = text.as_bytes();
    let len = s.len();
    let mut next = vec![vec![0; len]; len];
    for i in 0..len {
        let mut j = 0;
        for k in i + 1..len {
            while j > 0 && s[k] != s[i + j] {
                j = next[i][i + j - 1];
            }
            if s[k] == s[i + j] { j += 1; }
            next[i][k] = j;
        }
    }
    let mut dp = vec![vec![1; len]; len];
    for dlen in 1..len {
        for i in 0..len - dlen {
            let j = i + dlen;
            if next[i][j] > 0 {
                let i1 = i + next[i][j];
                let j1 = j - next[i][j];
                dp[i][j] = dp[i][j].max(if j1 >= i1 { dp[i1][j1] } else { 0 } + 2);
                if s[i] == s[j] {
                    dp[i][j] = dp[i][j].max(if j - 1 >= i + 1 { dp[i + 1][j - 1] } else { 0 } + 2);
                }
            }
        }
    }
    dp[0][len - 1]
}

fn main() {
    fn test(func: fn(text: String) -> i32) {
        assert_eq!(func(String::from("aaa")), 3);
        assert_eq!(func(String::from("ghiabcdefhelloadamhelloabcdefghi")), 7);
        assert_eq!(func(String::from("merchant")), 1);
        assert_eq!(func(String::from("aa")), 2);
        assert_eq!(func(String::from("antaprezatepzapreanta")), 11);
    }
    test(longest_decomposition);
}
