//! 找出出现至少三次的最长特殊子字符串 II

pub fn maximum_length(s: String) -> i32 {
    let len = s.len();
    let mut groups = vec![vec![]; 26];
    let mut last = 0;
    let s = s.as_bytes();
    for (i, &ch) in s.iter().enumerate().skip(1) {
        if ch != s[last] {
            groups[(s[last] - b'a') as usize].push((i - last) as i32);
            last = i;
        }
    }
    groups[(s[last] - b'a') as usize].push((len - last) as i32);
    let mut result = 0;
    for mut group in groups {
        if group.is_empty() { continue; }
        group.sort_unstable_by(|a, b| b.cmp(a));
        group.push(0);
        group.push(0);
        result = result.max(group[0] - 2).max(group[1].min(group[0] - 1)).max(group[2]);
    }
    if result == 0 { -1 } else { result }
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("ereerrrererrrererre")), 2);
        assert_eq!(func(String::from("aaaa")), 2);
        assert_eq!(func(String::from("abcdef")), -1);
        assert_eq!(func(String::from("abcaba")), 1);
    }
    test(maximum_length);
}
