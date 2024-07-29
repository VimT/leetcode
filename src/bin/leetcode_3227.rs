//! 字符串元音游戏

pub fn does_alice_win(s: String) -> bool {
    s.into_bytes().into_iter().any(|x| matches!(x, b'a' | b'e' | b'i' | b'o' | b'u'))
}

fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("leetcoder")), true);
        assert_eq!(func(String::from("bbcd")), false);
    }
    test(does_alice_win);
}
