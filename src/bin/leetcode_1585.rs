//! 检查字符串是否可以通过排序子字符串得到另一个字符串

/// 只交换相邻，从后往前，找到符合值，冒泡出来
pub fn is_transformable(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let len = s.len();
    let mut pos = vec![vec![]; 10];
    for (i, &ch) in s.iter().enumerate() {
        pos[(ch - b'0') as usize].push(i);
    }
    for i in (0..len).rev() {
        let ch = (t[i] - b'0') as usize;
        if let Some(last) = pos[ch].pop() {
            // 在 [last, i] 之间，不能出现 > t[i] 的值
            for j in ch + 1..10 {
                if !pos[j].is_empty() && *pos[j].last().unwrap() > last {
                    return false;
                }
            }
        } else { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(s: String, t: String) -> bool) {
        assert_eq!(func(String::from("84532"), String::from("34852")), true);
        assert_eq!(func(String::from("34521"), String::from("23415")), true);
        assert_eq!(func(String::from("12345"), String::from("12435")), false);
        assert_eq!(func(String::from("1"), String::from("2")), false);
    }
    test(is_transformable);
}
