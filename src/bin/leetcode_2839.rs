//! 判断通过操作能否让字符串相等 I

pub fn can_be_equal(s1: String, s2: String) -> bool {
    let mut s1 = s1.into_bytes();
    let mut s2 = s2.into_bytes();
    fn fix(s: &mut Vec<u8>) {
        if s[0] > s[2] { s.swap(0, 2); }
        if s[1] > s[3] { s.swap(1, 3); }
    }
    fix(&mut s1);
    fix(&mut s2);
    s1 == s2
}

fn main() {
    fn test(func: fn(s1: String, s2: String) -> bool) {
        assert_eq!(func(String::from("abcd"), String::from("cdab")), true);
        assert_eq!(func(String::from("abcd"), String::from("dacb")), false);
    }
    test(can_be_equal);
}
