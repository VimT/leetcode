//! 字典序最小的合法序列

pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
    let s1 = word1.as_bytes();
    let s2 = word2.as_bytes();
    let len1 = s1.len();
    let len2 = s2.len();
    let mut suffix = vec![0; len1 + 1];
    suffix[len1] = len2;
    let mut j = len2;
    for i in (0..len1).rev() {
        if j > 0 && s1[i] == s2[j - 1] {
            j -= 1;
        }
        suffix[i] = j;
    }
    let mut result = vec![];
    let mut changed = false;
    j = 0;
    for (i, &ch) in s1.iter().enumerate() {
        if ch == s2[j] || !changed && suffix[i + 1] <= j + 1 {
            if ch != s2[j] {
                changed = true;
            }
            result.push(i as i32);
            j += 1;
            if j == len2 { return result; }
        }
    }
    vec![]
}

fn main() {
    fn test(func: fn(word1: String, word2: String) -> Vec<i32>) {
        assert_eq!(func(String::from("vbcca"), String::from("abc")), vec![0, 1, 2]);
        assert_eq!(func(String::from("bacdc"), String::from("abc")), vec![1, 2, 4]);
        assert_eq!(func(String::from("aaaaaa"), String::from("aaabc")), vec![]);
        assert_eq!(func(String::from("abc"), String::from("ab")), vec![0, 1]);
    }
    test(valid_sequence);
}
