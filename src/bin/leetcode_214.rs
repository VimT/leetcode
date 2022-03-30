//! 最短回文串

pub fn shortest_palindrome(s: String) -> String {
    if s.len() == 0 { return s; }
    let chars = s.as_bytes();
    let mut i = s.len() - 1;
    while i > 0 {
        let mut left = 0;
        let mut right = i;
        while left < right && chars[left] == chars[right] {
            left += 1;
            right -= 1;
        }
        if left == right || left == right + 1 {
            break;
        }
        i -= 1;
    }
    let mut prefix = chars[i + 1..].to_vec();
    prefix.reverse();
    format!("{}{}", unsafe { String::from_utf8_unchecked(prefix) }, s)
}

/// 当s[i] == s[j] 时，[0,i) 必然包含最大回文串
pub fn shortest_palindrome_double_point(s: String) -> String {
    let n = s.len();
    let mut i = 0;
    let chars = s.as_bytes();
    for j in (0..n).rev() {
        if chars[i] == chars[j] { i += 1; }
    }
    if i == n { return s; }
    let mut remain_rev = chars[i..n].to_vec();
    remain_rev.reverse();
    format!("{}{}{}", unsafe { String::from_utf8_unchecked(remain_rev) }, shortest_palindrome_double_point(s[..i].to_string()), &s[i..])
}

/// kmp
pub fn shortest_palindrome_kmp(s: String) -> String {
    let n = s.len();
    let rev = s.chars().rev().collect::<String>();
    let news = format!("{}#{}", s, rev);
    let newc = news.as_bytes();
    let nn = news.len();
    let mut f = vec![0; nn];
    for i in 1..nn {
        let mut t = f[i - 1];
        while t > 0 && newc[i] != newc[t] { t = f[t - 1]; }
        if newc[i] == newc[t] { t += 1; }
        f[i] = t;
    }
    format!("{}{}", &rev[..n - f[nn - 1]], s)
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("aacecaaa")), String::from("aaacecaaa"));
        assert_eq!(func(String::from("abcd")), String::from("dcbabcd"));
    }
    test(shortest_palindrome);
    test(shortest_palindrome_double_point);
    test(shortest_palindrome_kmp);
}
