//! 哈希分割字符串

pub fn string_hash(s: String, k: i32) -> String {
    String::from_utf8(s.into_bytes().chunks(k as usize).map(|s| {
        (s.iter().map(|&ch| (ch - b'a') as i32).sum::<i32>() % 26) as u8 + b'a'
    }).collect::<Vec<u8>>()).unwrap()
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("abcd"), 2), String::from("bf"));
        assert_eq!(func(String::from("mxz"), 3), String::from("i"));
    }
    test(string_hash);
}
