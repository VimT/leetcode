//! 单词规律 II

/// 回溯
pub fn word_pattern_match(pattern: String, s: String) -> bool {
    fn dfs<'a>(p: &[u8], s: &'a [u8], m: &mut [&'a [u8]], si: usize, pi: usize) -> bool {
        if si == s.len() && pi == p.len() { return true; }
        if si == s.len() || pi == p.len() { return false; }
        let mi = (p[pi] - b'a') as usize;
        if !m[mi].is_empty() {
            let end = si + m[mi].len();
            return end <= s.len() && &s[si..end] == m[mi] && dfs(p, s, m, end, pi + 1);
        }
        for end in si..s.len() {
            // 检查 这段字符串是不是已经被别的 模式字符占用了
            let mut dup = false;
            for i in 0..26 {
                if m[i] == &s[si..=end] {
                    dup = true;
                    break;
                }
            }
            if dup { continue; }
            m[mi] = &s[si..=end];
            if dfs(p, s, m, end + 1, pi + 1) { return true; }
            m[mi] = &s[s.len()..];
        }
        false
    }
    dfs(pattern.as_bytes(), s.as_bytes(), &mut [&b""[..]; 26], 0, 0)
}

fn main() {
    fn test(func: fn(pattern: String, s: String) -> bool) {
        assert_eq!(func(String::from("abab"), String::from("redblueredblue")), true);
        assert_eq!(func(String::from("aaaa"), String::from("asdasdasdasd")), true);
        assert_eq!(func(String::from("aabb"), String::from("xyzabcxzyabc")), false);
        assert_eq!(func(String::from("ab"), String::from("aa")), false);
    }
    test(word_pattern_match);
}
