//! K 次操作转变字符串

pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
    if s.len() != t.len() { return false; }
    let len = s.len();
    let mut cnt = [0; 26];
    let s = s.as_bytes();
    let t = t.as_bytes();
    for i in 0..len {
        let diff = (t[i] + 26 - s[i]) % 26;
        cnt[diff as usize] += 1;
    }
    for i in 1..26 {
        if cnt[i] > 0 {
            let can = k / 26 + ((i as i32) <= k % 26) as i32;
            if can < cnt[i] { return false; }
        }
    }
    true
}

fn main() {
    fn test(func: fn(s: String, t: String, k: i32) -> bool) {
        assert_eq!(func(String::from("aab"), String::from("bbb"), 27), true);
        assert_eq!(func(String::from("input"), String::from("ouput"), 9), true);
        assert_eq!(func(String::from("abc"), String::from("bcd"), 10), false);
    }
    test(can_convert_string);
}
