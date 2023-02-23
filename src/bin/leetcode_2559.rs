//! 统计范围内的元音字符串数

fn is_vowel(ch: u8) -> bool {
    return matches!(ch, b'a'|b'e'|b'i'|b'o'|b'u');
}

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = words.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        let w = words[i].as_bytes();
        let is = is_vowel(w[0]) && is_vowel(w[w.len() - 1]);
        presum[i + 1] = presum[i] + is as i32;
    }
    queries.into_iter().map(|query| {
        presum[query[1] as usize + 1] - presum[query[0] as usize]
    }).collect()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(svec!["aba","bcb","ece","aa","e"], vec![vec![0, 2], vec![1, 4], vec![1, 1]]), vec![2, 3, 0]);
        assert_eq!(func(svec!["a","e","i"], vec![vec![0, 2], vec![0, 1], vec![2, 2]]), vec![3, 2, 1]);
    }
    test(vowel_strings);
}
