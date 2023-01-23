//! 制造字母异位词的最小步骤数

pub fn min_steps(s: String, t: String) -> i32 {
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for &ch in s.as_bytes() {
        c1[(ch - b'a') as usize] += 1;
    }
    for &ch in t.as_bytes() {
        c2[(ch - b'a') as usize] += 1;
    }
    (0..26).map(|x| (c2[x] - c1[x]).max(0)).sum()
}

fn main() {
    fn test(func: fn(s: String, t: String) -> i32) {
        assert_eq!(func(String::from("bab"), String::from("aba")), 1);
        assert_eq!(func(String::from("leetcode"), String::from("practice")), 5);
        assert_eq!(func(String::from("anagram"), String::from("mangaar")), 0);
    }
    test(min_steps);
}
