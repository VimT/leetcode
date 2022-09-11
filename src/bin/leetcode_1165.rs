//! 单行键盘

pub fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut map = vec![0; 128];
    let x = keyboard.as_bytes();
    for (i, &ch) in x.iter().enumerate() {
        map[ch as usize] = i as i32;
    }
    let mut last = 0;
    let mut result = 0;
    for &ch in word.as_bytes() {
        let pos = map[ch as usize];
        result += (pos - last).abs();
        last = pos;
    }
    result
}

fn main() {
    fn test(func: fn(keyboard: String, word: String) -> i32) {
        assert_eq!(func(String::from("abcdefghijklmnopqrstuvwxyz"), String::from("cba")), 4);
        assert_eq!(func(String::from("pqrstuvwxyzabcdefghijklmno"), String::from("leetcode")), 73);
    }
    test(calculate_time);
}
