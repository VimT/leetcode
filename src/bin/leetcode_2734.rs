//! 执行子串操作后的字典序最小字符串

pub fn smallest_string(mut s: String) -> String {
    let ss = unsafe { s.as_bytes_mut() };
    let len = ss.len();
    let mut change = false;
    for i in 0..len {
        if ss[i] != b'a' {
            let mut j = i;
            while j < len && ss[j] != b'a' {
                ss[j] -= 1;
                j += 1;
            }
            change = true;
            break;
        }
    }
    if !change {
        ss[len - 1] = b'z';
    }
    s
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("a")), String::from("z"));
        assert_eq!(func(String::from("cbabc")), String::from("baabc"));
        assert_eq!(func(String::from("acbbc")), String::from("abaab"));
        assert_eq!(func(String::from("leetcode")), String::from("kddsbncd"));
    }
    test(smallest_string);
}
