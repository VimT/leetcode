//! 上升下降字符串

pub fn sort_string(s: String) -> String {
    let mut cnt = [0; 26];
    let len = s.len();
    for &ch in s.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut result = Vec::with_capacity(len);
    while result.len() < len {
        for i in (0..26).chain((0..26).rev()) {
            if cnt[i] > 0 {
                result.push(i as u8 + b'a');
                cnt[i] -= 1;
            }
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("aaaabbbbcccc")), String::from("abccbaabccba"));
        assert_eq!(func(String::from("rat")), String::from("art"));
    }
    test(sort_string);
}
