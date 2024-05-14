//! 两个字符串的排列差

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut pos1 = [0; 26];
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        pos1[(ch - b'a') as usize] = i;
    }
    let mut pos2 = [0; 26];
    for (i, &ch) in t.as_bytes().iter().enumerate() {
        pos2[(ch - b'a') as usize] = i;
    }
    pos1.iter().zip(pos2.iter()).map(|(&a, &b)| a.abs_diff(b) as i32).sum()
}

fn main() {
    fn test(func: fn(s: String, t: String) -> i32) {
        assert_eq!(func(String::from("rwohu"), String::from("rwuoh")), 4);
        assert_eq!(func(String::from("abc"), String::from("bac")), 2);
        assert_eq!(func(String::from("abcde"), String::from("edbac")), 12);
    }
    test(find_permutation_difference);
}
