//! 分割两个字符串得到回文串

/// 看a的前缀和b的后缀的最大匹配数量，再看a/b剩下的能否组成回文串
pub fn check_palindrome_formation(a: String, b: String) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let len = a.len();
    let mut i = 0;
    let mut j = 0;
    while i < len / 2 && a[i] == b[len - 1 - i] {
        i += 1;
    }
    while j < len / 2 && b[j] == a[len - 1 - j] {
        j += 1;
    }
    i = i.max(j);
    fn is_palindrome(s: &[u8]) -> bool {
        let len = s.len();
        for i in 0..len / 2 {
            if s[i] != s[len - i - 1] {
                return false;
            }
        }
        true
    }
    is_palindrome(&a[i..len - i]) || is_palindrome(&b[i..len - i])
}

fn main() {
    fn test(func: fn(a: String, b: String) -> bool) {
        assert_eq!(func(String::from("abda"), String::from("acmc")), false);
        assert_eq!(func(String::from("abdef"), String::from("fecab")), true);
        assert_eq!(func(String::from("x"), String::from("y")), true);
        assert_eq!(func(String::from("xbdef"), String::from("xecab")), false);
        assert_eq!(func(String::from("ulacfd"), String::from("jizalu")), true);
    }
    test(check_palindrome_formation);
}
