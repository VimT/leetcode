//! 回文字符串的最大数量

/// 对于奇数长度的字符串，先填两边的
pub fn max_palindromes_after_operations(mut words: Vec<String>) -> i32 {
    let mut result = 0;
    let mut tot = 0;
    let mut cnt = [0; 26];
    for w in &words {
        tot += w.len() as i32;
        for &ch in w.as_bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
    }
    for i in 0..26 {
        tot -= cnt[i] & 1; // 减去出现次数为奇数的字母
    }
    words.sort_unstable_by_key(|x| x.len());
    for w in words {
        let wlen = w.len() as i32;
        tot -= wlen - (wlen & 1);
        if tot < 0 { break; }
        result += 1;
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>) -> i32) {
        assert_eq!(func(svec!["abbb","ba","aa"]), 3);
        assert_eq!(func(svec!["abc","ab"]), 2);
        assert_eq!(func(svec!["cd","ef","a"]), 1);
    }
    test(max_palindromes_after_operations);
}
