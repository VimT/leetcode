//! 统计重新排列后包含另一个字符串的子字符串数目 II

pub fn valid_substring_count(word1: String, word2: String) -> i64 {
    if word1.len() < word2.len() {
        return 0;
    }
    let mut cnt = [0; 26];
    for &ch in word2.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut less = (0..26).filter(|&i| cnt[i] > 0).count();
    let mut l = 0;
    let s = word1.as_bytes();
    let len = s.len();
    let mut result = 0;
    for r in 0..len {
        let idx = (s[r] - b'a') as usize;
        cnt[idx] -= 1;
        if cnt[idx] == 0 {
            less -= 1;
        }
        while less == 0 {
            if cnt[(s[l] - b'a') as usize] == 0 {
                less += 1;
            }
            cnt[(s[l] - b'a') as usize] += 1;
            l += 1;
        }
        result += l;
    }
    result as i64
}

fn main() {
    fn test(func: fn(word1: String, word2: String) -> i64) {
        assert_eq!(func(String::from("bcca"), String::from("abc")), 1);
        assert_eq!(func(String::from("abcabc"), String::from("abc")), 10);
        assert_eq!(func(String::from("abcabc"), String::from("aaabc")), 0);
    }
    test(valid_substring_count);
}
