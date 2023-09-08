//! 循环增长使字符串子序列等于另一个字符串

pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    let mut i = 0;
    for &ch in s1 {
        let c = if ch == b'z' { b'a' } else { ch + 1 };
        if ch == s2[i] || c == s2[i] {
            i += 1;
            if i == s2.len() {
                return true;
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(str1: String, str2: String) -> bool) {
        assert_eq!(func(String::from("abc"), String::from("ad")), true);
        assert_eq!(func(String::from("zc"), String::from("ad")), true);
        assert_eq!(func(String::from("ab"), String::from("d")), false);
    }
    test(can_make_subsequence);
}
