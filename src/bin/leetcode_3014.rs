//! 输入单词需要的最少按键次数 I

pub fn minimum_pushes(word: String) -> i32 {
    let mut cnt = [0; 26];
    for &ch in word.as_bytes() {
        cnt[(ch - b'a') as usize] += 1;
    }
    cnt.sort_unstable();
    let mut result = 0;

    for (&num, i) in cnt.iter().rev().zip(1..) {
        result += num * ((i + 7) / 8);
    }
    result
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("abcde")), 5);
        assert_eq!(func(String::from("xycdefghij")), 12);
    }
    test(minimum_pushes);
}
