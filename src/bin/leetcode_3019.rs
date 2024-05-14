//! 按键变更的次数

pub fn count_key_changes(s: String) -> i32 {
    s.as_bytes().windows(2).filter(|x| x[0].to_ascii_lowercase() != x[1].to_ascii_lowercase()).count() as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("aAbBcC")), 2);
        assert_eq!(func(String::from("AaAaAaaA")), 0);
    }
    test(count_key_changes);
}
