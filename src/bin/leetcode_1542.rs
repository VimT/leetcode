//! 找出最长的超赞子字符串

/// 位运算，哈希表
pub fn longest_awesome(s: String) -> i32 {
    let s = s.as_bytes();
    let mut last = vec![None; 1 << 10];
    last[0] = Some(0);
    let mut cur = 0;
    let mut result = 0;
    for (&ch, i) in s.iter().zip(1..) {
        cur ^= 1 << (ch - b'0');
        if let Some(j) = last[cur] {
            result = result.max(i - j);
        }
        for b in 0..10 {
            if let Some(j) = last[cur ^ 1 << b] {
                result = result.max(i - j);
            }
        }
        if last[cur] == None {
            last[cur] = Some(i);
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("373781")), 5);
        assert_eq!(func(String::from("213123")), 6);
        assert_eq!(func(String::from("3242415")), 5);
        assert_eq!(func(String::from("12345678")), 1);
        assert_eq!(func(String::from("00")), 2);
    }
    test(longest_awesome);
}
