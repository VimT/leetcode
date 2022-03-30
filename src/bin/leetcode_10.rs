//! 正则表达式匹配

use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    fn dfs(s: &Vec<char>, s_idx: usize, p: &Vec<char>, p_idx: usize, cache: &mut HashMap<(usize, usize), bool>) -> bool {
        if cache.contains_key(&(s_idx, p_idx)) {
            return *cache.get(&(s_idx, p_idx)).unwrap();
        }
        if p_idx == p.len() { return s_idx == s.len(); }
        let first_match = s_idx != s.len() && (s[s_idx] == p[p_idx] || p[p_idx] == '.');
        let ans = if p_idx + 1 < p.len() && p[p_idx + 1] == '*' {
            dfs(s, s_idx, p, p_idx + 2, cache) || (first_match && dfs(s, s_idx + 1, p, p_idx, cache))
        } else {
            first_match && dfs(s, s_idx + 1, p, p_idx + 1, cache)
        };
        cache.insert((s_idx, p_idx), ans);
        return ans;
    }
    let mut mem = HashMap::new();
    let s = s.chars().collect();
    let p = p.chars().collect();
    return dfs(&s, 0, &p, 0, &mut mem);
}

/// dp[i][j] 表示s[i:] 和 p[j:] 是否匹配
pub fn is_match_dp(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let slen = s.len();
    let plen = p.len();
    let mut dp = vec![vec![false; plen + 1]; slen + 1];
    dp[0][0] = true;
    for i in 0..=slen {
        for j in 1..=plen {
            dp[i][j] = if p[j - 1] == b'*' {
                (i > 0 && (s[i - 1] == p[j - 2] || p[j - 2] == b'.') && dp[i - 1][j]) || dp[i][j - 2]
            } else {
                i > 0 && (s[i - 1] == p[j - 1] || p[j - 1] == b'.') && dp[i - 1][j - 1]
            }
        }
    }
    dp[slen][plen]
}


fn main() {
    fn test(func: fn(s: String, p: String) -> bool) {
        assert_eq!(func(String::from("aa"), String::from("a")), false);
        assert_eq!(func(String::from("aa"), String::from("a*")), true);
        assert_eq!(func(String::from("ab"), String::from(".*")), true);
    }
    test(is_match);
    test(is_match_dp);
}
