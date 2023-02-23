//! 不同的循环子字符串

use std::collections::HashSet;

const BASE: u64 = 1331;
const MOD: u64 = 1e9 as u64 + 7;

pub fn distinct_echo_substrings(text: String) -> i32 {
    let s = text.as_bytes();
    let len = s.len();
    let mut hash = vec![vec![0; len]; len];
    for i in 0..len {
        let mut cur = 0;
        for j in i..len {
            cur = (cur * BASE + s[j] as u64) % MOD;
            hash[i][j] = cur;
        }
    }
    let mut result = HashSet::new();
    for i in 0..len {
        let mut j = i + 1;
        let mut mid = i;
        while j < len {
            if hash[i][mid] == hash[mid + 1][j] {
                result.insert(hash[i][mid]);
            }
            mid += 1;
            j += 2;
        }
    }

    result.len() as i32
}

/// KMP
pub fn distinct_echo_substrings2(text: String) -> i32 {
    fn kmp(s: &[u8], vis: &mut HashSet<u64>) {
        let len = s.len();
        let mut pi = vec![0; len]; // pi[i]表示s[..=i]最大的相等的真前缀和真后缀的长度。
        let mut j = 0;
        let mut cur = s[0] as u64;
        for i in 1..len {
            while j > 0 && s[i] != s[j] {
                j = pi[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            pi[i] = j;
            // 上面是kmp模板

            cur = (cur * BASE + s[j] as u64) % MOD;
            let m = i + 1; // 当前字符串长度
            let unit = m - j;
            if m & 1 == 0 && m % unit == 0 && (m / unit) & 1 == 0 {
                vis.insert(cur);
            }
        }
    }
    let s = text.as_bytes();
    let len = s.len();
    let mut vis = HashSet::new();
    for i in 0..len - 1 {
        kmp(&s[i..], &mut vis);
    }
    vis.len() as i32
}

fn main() {
    fn test(func: fn(text: String) -> i32) {
        assert_eq!(func(String::from("aaaaaaaaaa")), 5);
        assert_eq!(func(String::from("abcabcabc")), 3);
        assert_eq!(func(String::from("leetcodeleetcode")), 2);
    }
    test(distinct_echo_substrings);
    test(distinct_echo_substrings2);
}
