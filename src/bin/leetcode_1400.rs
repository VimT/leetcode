//! 构造 K 个回文字符串

pub fn can_construct(s: String, k: i32) -> bool {
    let s = s.as_bytes();
    let len = s.len();
    let k = k as usize;
    if len < k {
        return false;
    }
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut single = 0;
    for i in 0..26 {
        if cnt[i] & 1 == 1 {
            single += 1;
        }
    }
    single <= k
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> bool) {
        assert_eq!(func(String::from("annabelle"), 2), true);
        assert_eq!(func(String::from("leetcode"), 3), false);
        assert_eq!(func(String::from("true"), 4), true);
        assert_eq!(func(String::from("cr"), 7), false);
    }
    test(can_construct);
}
