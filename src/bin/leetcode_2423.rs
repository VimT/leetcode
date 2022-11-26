//! 删除字符使频率相同


use std::collections::HashSet;

pub fn equal_frequency(word: String) -> bool {
    let mut freq = [0; 26];
    for &ch in word.as_bytes() {
        freq[(ch - b'a') as usize] += 1;
    }
    // 模拟扣减
    for i in 0..26 {
        if freq[i] > 0 {
            freq[i] -= 1;
            if freq.iter().cloned().filter(|x| *x > 0).collect::<HashSet<i32>>().len() == 1 { return true; }
            freq[i] += 1;
        }
    }
    false
}

fn main() {
    fn test(func: fn(word: String) -> bool) {
        assert_eq!(func(String::from("cbccca")), false);
        assert_eq!(func(String::from("abbcc")), true);
        assert_eq!(func(String::from("bac")), true);
        assert_eq!(func(String::from("abcc")), true);
        assert_eq!(func(String::from("aazz")), false);
    }
    test(equal_frequency);
}
