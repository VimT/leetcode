//! 兼具大小写的最好英文字母

pub fn greatest_letter(s: String) -> String {
    let mut seen = [false; 128];
    for &ch in s.as_bytes() {
        seen[ch as usize] = true;
    }
    for i in (b'A'..=b'Z').rev() {
        if seen[i as usize] && seen[(i - b'A' + b'a') as usize] {
            return (i as char).to_string();
        }
    }
    String::new()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("lEeTcOdE")), String::from("E"));
        assert_eq!(func(String::from("arRAzFif")), String::from("R"));
        assert_eq!(func(String::from("AbCdEfGhIjK")), String::from(""));
    }
    test(greatest_letter);
}
