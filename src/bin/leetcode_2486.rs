//! 追加字符以获得子序列

pub fn append_characters(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut ti = 0;
    for &ch in s {
        if ti == t.len() { break; }
        if ch == t[ti] {
            ti += 1;
        }
    }
    (t.len() - ti) as i32
}

fn main() {
    fn test(func: fn(s: String, t: String) -> i32) {
        assert_eq!(func(String::from("coaching"), String::from("coding")), 4);
        assert_eq!(func(String::from("abcde"), String::from("a")), 0);
        assert_eq!(func(String::from("z"), String::from("abcde")), 5);
    }
    test(append_characters);
}
