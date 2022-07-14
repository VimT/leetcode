//! 最大波动的子字符串

/// 枚举最大和最小字符
pub fn largest_variance(s: String) -> i32 {
    let s = s.as_bytes();
    let mut result = 0;
    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            if a == b { continue; }
            // 以a结尾的连续个数
            let mut p1 = 0;
            // 有a有b的字串波动值
            let mut p2 = i32::MIN / 2;
            for &ch in s {
                if ch == a {
                    p1 += 1;
                    p2 += 1;
                } else if ch == b {
                    p2 = (p1 - 1).max(p2 - 1);
                    p1 = 0;
                }
                result = result.max(p2);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("icexiahccknibwuwgi")), 3);
        assert_eq!(func(String::from("aababbb")), 3);
        assert_eq!(func(String::from("abcde")), 0);
    }
    test(largest_variance);
}
