//! 字符串的总引力

/// 想每个字母，对 以当前遍历字母i为结尾的 所以子字符串，贡献有多少？
pub fn appeal_sum(s: String) -> i64 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    let mut last_seen = [usize::MAX; 26];
    for i in 0..len {
        last_seen[(s[i] - b'a') as usize] = i + 1;
        let mut cnt = 0;
        for i in 0..26 {
            if last_seen[i] != usize::MAX {
                cnt += last_seen[i];
            }
        }
        result += cnt as i64;
    }
    result
}

/// 优化，新字母 比上个字母的贡献，多出来的是 上次出现位置到现在位置。
pub fn appeal_sum2(s: String) -> i64 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    let mut sum = 0;
    let mut last_seen = [-1; 26];
    for i in 0..len {
        sum += i as i64 - last_seen[(s[i] - b'a') as usize];
        result += sum;
        last_seen[(s[i] - b'a') as usize] = i as i64;
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i64) {
        assert_eq!(func(String::from("abbca")), 28);
        assert_eq!(func(String::from("code")), 20);
    }
    test(appeal_sum);
    test(appeal_sum2);
}
