//! 最长快乐前缀

/// KMP
pub fn longest_prefix(ss: String) -> String {
    let s = ss.as_bytes();
    let len = s.len();
    let mut next = vec![0; len];
    let mut j = 0;
    for i in 1..len {
        // why while: aabaaa, last a need while
        while j > 0 && s[i] != s[j] {
            j = next[j - 1];
        }
        if s[i] == s[j] { j += 1; }
        next[i] = j;
    }
    ss[..next[len - 1]].to_string()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("level")), String::from("l"));
        assert_eq!(func(String::from("ababab")), String::from("abab"));
    }
    test(longest_prefix);
}
