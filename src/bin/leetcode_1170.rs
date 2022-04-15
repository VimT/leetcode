//! 比较字符串最小字母出现频次

use leetcode::svec;

pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    fn cal(s: &String) -> i32 {
        let mut cnt = [0; 26];
        for &ch in s.as_bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if cnt[i] != 0 { return cnt[i]; }
        }
        0
    }
    let mut word_cnt: Vec<i32> = words.iter().map(|x| cal(x)).collect();
    word_cnt.sort_unstable();
    queries.iter().map(|x| {
        let cnt = cal(x);
        let mut left = 0;
        let mut right = word_cnt.len();
        while left < right {
            let mid = (left + right) / 2;
            if word_cnt[mid] > cnt {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        (word_cnt.len() - left) as i32
    }).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<String>, words: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["cbd"], svec!["zaaaz"]), vec![1]);
        assert_eq!(func(svec!["bbb","cc"], svec!["a","aa","aaa","aaaa"]), vec![1, 2]);
    }
    test(num_smaller_by_frequency);
}
