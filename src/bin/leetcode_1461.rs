//! 检查一个字符串是否包含所有长度为 K 的二进制子串

pub fn has_all_codes(s: String, k: i32) -> bool {
    let s = s.as_bytes();
    let k = k as usize;
    if s.len() < k { return false; }
    let mut cnt = vec![false; 1 << k];
    let mut cur = 0;
    for &ch in &s[..k] {
        cur = (cur << 1) + (ch - b'0') as usize;
    }
    cnt[cur] = true;
    for i in k..s.len() {
        cur = (cur << 1) + (s[i] - b'0') as usize;
        cur ^= ((s[i - k] - b'0') as usize) << k;
        cnt[cur] = true;
    }
    for i in 0..1 << k {
        if !cnt[i] {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> bool) {
        assert_eq!(func(String::from("0"), 20), false);
        assert_eq!(func(String::from("00110110"), 2), true);
        assert_eq!(func(String::from("0110"), 1), true);
        assert_eq!(func(String::from("0110"), 2), false);
    }
    test(has_all_codes);
}
