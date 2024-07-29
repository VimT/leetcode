//! 操作后字符串的最短长度

pub fn minimum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut result = 0;
    for i in 0..26 {
        if cnt[i] > 0 {
            result += if cnt[i] % 2 == 1 { 1 } else { 2 };
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abaacbcbb")), 5);
        assert_eq!(func(String::from("aa")), 2);
    }
    test(minimum_length);
}
