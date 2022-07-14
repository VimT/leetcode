//! 最长重复子串

use std::collections::HashSet;

/// dp[i][j] 表示以i结尾和以j结尾的重复字串的长度
pub fn longest_repeating_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![vec![0; len + 1]; len + 1];
    let mut result = 0;
    for i in 1..=len {
        for j in i + 1..=len {
            if s[i - 1] == s[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                result = result.max(dp[i][j]);
            }
        }
    }
    result
}


/// 二分查找+滚动hash
pub fn longest_repeating_substring2(s: String) -> i32 {
    const BASE: u64 = 113;
    const MOD: u64 = 1e9 as u64 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut left = 1;
    let mut right = len;
    let mut seen = HashSet::new();
    while left < right {
        seen.clear();
        let mid = (left + right) / 2;
        let mut hash = 0;
        let mut pow = 1;
        for i in 0..mid {
            hash = (hash * BASE + s[i] as u64) % MOD;
            pow = (pow * BASE) % MOD;
        }
        seen.insert(hash);
        let mut ok = false;
        for i in mid..len {
            hash = (hash * BASE - s[i - mid] as u64 * pow % MOD + MOD + s[i] as u64) % MOD;
            if seen.contains(&hash) {
                ok = true;
                break;
            }
            seen.insert(hash);
        }
        if ok {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32 - 1
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abbaba")), 2);
        assert_eq!(func(String::from("abcd")), 0);
        assert_eq!(func(String::from("aabcaabdaab")), 3);
    }
    test(longest_repeating_substring);
    test(longest_repeating_substring2);
}
