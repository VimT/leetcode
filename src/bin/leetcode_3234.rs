//! 统计 1 显著的字符串的数量

/// 以 left 为子字符串左边界，枚举子字符串0的数量 （最多 sqrt(n) 个）
pub fn number_of_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut zero: Vec<usize> = (0..len).filter(|&x| s[x] == b'0').collect();
    let total_one = len - zero.len();
    zero.push(len); // 哨兵
    let mut result = 0;
    let mut i = 0;
    for (left, &ch) in s.iter().enumerate() {
        if ch == b'1' { result += zero[i] - left; } // zero_cnt = 0， zero[i] 为下一个0的位置
        for k in i..zero.len() - 1 {
            let cnt0 = k - i + 1; // 子字符串0的数量
            if cnt0 * cnt0 > total_one { break; }
            let cnt1 = zero[k] - left - (k - i); // 子字符串1的数量
            let mid = zero[k + 1] - zero[k]; // 0 的数量为 cnt0 的字符串数量
            if cnt0 * cnt0 <= cnt1 {
                result += mid;
            } else {
                result += mid.saturating_sub(cnt0 * cnt0 - cnt1); // 需要 cnt0^2-cnt1 个1去填坑
            }
        }
        if ch == b'0' { i += 1; }
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("00011")), 5);
        assert_eq!(func(String::from("101101")), 16);
    }
    test(number_of_substrings);
}
