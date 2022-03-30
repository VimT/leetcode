//! 同源字符串检测

use std::collections::HashSet;

/// dp[i][j]dp[i][j]dp[i][j]表示将s1的前i个字母和s2的前j个字母匹配且不发生冲突时，可能的长度差值。
pub fn possibly_equals(s1: String, s2: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let len1 = s1.len();
    let len2 = s2.len();
    let mut dp = vec![vec![HashSet::new(); len2 + 1]; len1 + 1];
    dp[0][0].insert(0);
    for i in 0..=len1 {
        for j in 0..=len2 {
            let cur_delta: Vec<i32> = dp[i][j].iter().map(|x| *x).collect();
            for delta in cur_delta {
                let mut num = 0;
                for p in i..len1.min(i + 3) {
                    if s1[p].is_ascii_digit() {
                        num = num * 10 + (s1[p] - b'0') as i32;
                        dp[p + 1][j].insert(delta + num);
                    } else {
                        break;
                    }
                }
                num = 0;
                for q in j..len2.min(j + 3) {
                    if s2[q].is_ascii_digit() {
                        num = num * 10 + (s2[q] - b'0') as i32;
                        dp[i][q + 1].insert(delta - num);
                    } else {
                        break;
                    }
                }
                if i < len1 && delta < 0 && !s1[i].is_ascii_digit() {
                    dp[i + 1][j].insert(delta + 1);
                }
                if j < len2 && delta > 0 && !s2[j].is_ascii_digit() {
                    dp[i][j + 1].insert(delta - 1);
                }
                if i < len1 && j < len2 && delta == 0 && s1[i] == s2[j] {
                    dp[i + 1][j + 1].insert(0);
                }
            }
        }
    }
    dp[len1][len2].contains(&0)
}

fn main() {
    assert!(possibly_equals("l123e".to_string(), "44".to_string()));
    assert!(possibly_equals("internationalization".to_string(), "i18n".to_string()));
    assert!(!possibly_equals("a5b".to_string(), "c5b".to_string()));
    assert!(possibly_equals("112s".to_string(), "g841".to_string()));
    assert!(!possibly_equals("ab".to_string(), "a2".to_string()));
}
