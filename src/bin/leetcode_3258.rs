//! 统计满足 K 约束的子字符串数量 I

/// 滑动窗口
pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    let k = k as usize;
    let mut l = 0;
    let mut one = 0;
    for r in 0..len {
        if s[r] == b'1' { one += 1; }
        while one > k && (r - l + 1 - one) > k {
            if s[l] == b'1' { one -= 1; }
            l += 1;
        }
        result += r - l + 1;
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("10101"), 1), 12);
        assert_eq!(func(String::from("1010101"), 2), 25);
        assert_eq!(func(String::from("11111"), 1), 15);
    }
    test(count_k_constraint_substrings);
}
