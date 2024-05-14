//! 成为 K 特殊字符串需要删除的最少字符数

pub fn minimum_deletions(word: String, k: i32) -> i32 {
    let mut cnt = vec![0; 26];
    for &ch in word.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    cnt = cnt.into_iter().filter(|x| *x > 0).collect();
    cnt.sort_unstable();
    let mut result = i32::MAX;
    for i in 0..cnt.len() { // cnt[i] 是最小
        let mut this: i32 = cnt[..i].iter().sum();
        this += cnt[i..].iter().map(|&x| (x - k - cnt[i]).max(0)).sum::<i32>();
        result = result.min(this);
    }
    result
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("aabcaba"), 0), 3);
        assert_eq!(func(String::from("dabdcbdcdcd"), 2), 2);
        assert_eq!(func(String::from("aaabaaa"), 2), 1);
    }
    test(minimum_deletions);
}
