//! 分割连接字符串

use leetcode::svec;

pub fn split_looped_string(strs: Vec<String>) -> String {
    let strs: Vec<String> = strs.into_iter().map(|x| {
        let s = x.as_bytes();
        let mut left = 0;
        let mut right = s.len();
        let mut reverse = false;
        while left < right {
            if s[left] != s[right - 1] {
                reverse = s[right - 1] > s[left];
                break;
            }
            left += 1;
            right -= 1;
        }
        if reverse {
            let mut sr = s.to_vec();
            sr.reverse();
            return unsafe { String::from_utf8_unchecked(sr) };
        }
        x
    }).collect();
    let mut result = strs.join("");
    fn rev(s: &str) -> String {
        let mut s = s.as_bytes().to_vec();
        s.reverse();
        unsafe { String::from_utf8_unchecked(s) }
    }
    for (i, s) in strs.iter().enumerate() {
        let other = strs[i + 1..].join("") + &strs[..i].join("");
        let sr = rev(s);
        for j in 0..s.len() {
            result = result.max(s[j..].to_string() + &other + &s[..j]);
            result = result.max(sr[j..].to_string() + &other + &sr[..j]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(strs: Vec<String>) -> String) {
        assert_eq!(func(svec!["lc","evol","cdy"]), String::from("ylclovecd"));  // 考虑这个case
        assert_eq!(func(svec!["awef","eawf","zdaeff","awefzewaf","awefzewaf"]), String::from("zfewafewafwaezdaefffawezfewafawe"));
        assert_eq!(func(svec!["abc","xyz"]), String::from("zyxcba"));
        assert_eq!(func(svec!["abc"]), String::from("cba"));
    }
    test(split_looped_string);
}
