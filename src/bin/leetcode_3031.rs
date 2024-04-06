//! 将单词恢复初始状态所需的最短时间 II

use leetcode::algorithm::max_match_length;

/// Z函数：z[i] 表示 s[i..] 和 s[0..] 的最长公共前缀长度
pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let len = s.len();
    let k = k as usize;
    let mut z = vec![0; len];
    let mut l = 0;
    let mut r = 0;
    for i in 1..len {
        if i <= r {
            z[i] = z[i - l].min(r + 1 - i);
        }
        while i + z[i] < len && s[z[i]] == s[i + z[i]] {
            l = i;
            r = i + z[i];
            z[i] += 1;
        }
        if i % k == 0 && z[i] >= len - i {
            return (i / k) as i32;
        }
    }
    ((len + k - 1) / k) as i32
}

/// KMP 做法
pub fn minimum_time_to_initial_state2(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let len = s.len();
    let next = max_match_length(s);
    let k = k as usize;
    let mut index = len - 1;  // next[-1] 表示最长的公共前后缀长度，意味着最少要减去 前(len - next[-1])个字符
    while next[index] > 0 && (len - next[index]) % k != 0 {
        index = next[index] - 1;  // 失效匹配，回退。
    }
    ((len - next[index] + k - 1) / k) as i32
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("abacaba"), 3), 2);
        assert_eq!(func(String::from("abacaba"), 4), 1);
        assert_eq!(func(String::from("abcbabcd"), 2), 4);
    }
    test(minimum_time_to_initial_state);
    test(minimum_time_to_initial_state2);
}
