//! 连接两字母单词得到的最长回文串

use leetcode::svec;

pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut cnt = vec![0; 26 * 26];
    for word in words {
        let s = word.as_bytes();
        let idx = (s[0] - b'a') as usize * 26 + (s[1] - b'a') as usize;
        cnt[idx] += 1;
    }

    let mut result = 0;
    let mut mid = 0;
    for i in 0..26 * 26 {
        if cnt[i] > 0 {
            let a = i / 26;
            let b = i % 26;
            let rev = b * 26 + a;
            if a == b && cnt[i] & 1 == 1 {
                mid = 2;
                cnt[i] -= 1;
            }
            result += 2 * (cnt[i].min(cnt[rev]));
        }
    }
    result + mid
}

fn main() {
    assert_eq!(longest_palindrome(svec!["lc", "cl", "gg"]), 6);
    assert_eq!(longest_palindrome(svec!["ab", "ty", "yt", "lc", "cl", "ab"]), 8);
    assert_eq!(longest_palindrome(svec!["cc", "ll", "xx"]), 2);
}
