//! 通配符匹配

use std::collections::HashMap;

/// 和 leetcode_10 正则匹配类似
/// dp[i][j] 表示s[i:] 和 p[j:] 是否匹配
pub fn is_match(s: String, p: String) -> bool {
    let s = s.chars().collect();
    let p = p.chars().collect();

    fn dp(s: &Vec<char>, si: usize, p: &Vec<char>, pi: usize, cache: &mut HashMap<(usize, usize), bool>) -> bool {
        if cache.contains_key(&(si, pi)) {
            return *cache.get(&(si, pi)).unwrap();
        }
        if pi == p.len() { return si == s.len(); }
        if si > s.len() { return false; }
        let ans = if pi < p.len() && p[pi] == '*' {
            dp(s, si, p, pi + 1, cache) || dp(s, si + 1, p, pi, cache)
        } else {
            let first_match = si != s.len() && (s[si] == p[pi] || p[pi] == '?');
            first_match && dp(s, si + 1, p, pi + 1, cache)
        };
        cache.insert((si, pi), ans);
        ans
    }
    return dp(&s, 0, &p, 0, &mut HashMap::new());
}

/// dp[i][j] 表示 s[:i]和p[:j]是否匹配
/// dp[pi][si] = dp[pi-1][si-1] if p[i] == s[i] || p[i] == '?'
/// dp[pi][i] = true ( i>= si - 1 ) if p[pi-1] == '*' && dp[pi-1][si-1] = true
pub fn is_match_dp(s: String, p: String) -> bool {
    if s == p || p == "*".to_string() { return true; }
    if p.len() == 0 || s.len() == 0 { return false; }

    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();

    let p_len = p.len();
    let s_len = s.len();
    let mut dp = vec![vec![false; s_len + 1]; p_len + 1];
    dp[0][0] = true;

    for pi in 1..=p_len {
        if p[pi - 1] == '*' {
            let mut si = 1;
            while !dp[pi - 1][si - 1] && si < s_len + 1 {
                si += 1;
            }
            // 什么都没有匹配
            dp[pi][si - 1] = dp[pi - 1][si - 1];
            // 什么都匹配
            while si <= s_len {
                dp[pi][si] = true;
                si += 1;
            }
        } else if p[pi - 1] == '?' {
            for si in 1..=s_len {
                dp[pi][si] = dp[pi - 1][si - 1];
            }
        } else {
            for si in 1..=s_len {
                dp[pi][si] = dp[pi - 1][si - 1] && p[pi - 1] == s[si - 1];
            }
        }
    }
    dp[p_len][s_len]
}

/// 回溯
pub fn is_match_best(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();

    let s_len = s.len();
    let p_len = p.len();

    let mut si = 0;
    let mut pi = 0;

    let mut star_idx = s_len;
    let mut s_tmp_idx = 0;

    while si < s_len {
        if pi < p_len && (p[pi] == '?' || p[pi] == s[si]) {
            si += 1;
            pi += 1;
        } else if pi < p_len && p[pi] == '*' {
            star_idx = pi;
            s_tmp_idx = si;
            pi += 1;
        } else if star_idx == s_len {
            return false;
        } else {
            pi = star_idx + 1;
            si = s_tmp_idx + 1;
            s_tmp_idx = si;
        }
    }

    for i in pi..p_len {
        if p[i] != '*' {
            return false;
        }
    }

    return true;
}

fn main() {
    fn test(func: fn(s: String, p: String) -> bool) {
        assert_eq!(func(String::from("aa"), String::from("a")), false);
        assert_eq!(func(String::from("aa"), String::from("*")), true);
        assert_eq!(func(String::from("cb"), String::from("?a")), false);
    }
    test(is_match);
    test(is_match_best);
    test(is_match_dp);
}
