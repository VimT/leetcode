//! 统计特殊字母的数量 II

pub fn number_of_special_chars(word: String) -> i32 {
    #[derive(Eq, PartialEq, Copy, Clone)]
    enum State {
        Init,
        Lowercase,
        Uppercase,
        Invalid,
    }
    use State::*;
    let s = word.as_bytes();
    let mut state = [Init; 26];
    for &ch in s {
        if ch.is_ascii_lowercase() {
            let idx = (ch - b'a') as usize;
            match state[idx] {
                Init => state[idx] = Lowercase,
                Uppercase => state[idx] = Invalid,
                _ => {}
            }
        } else {
            let idx = (ch - b'A') as usize;
            match state[idx] {
                Init => state[idx] = Invalid,
                Lowercase => state[idx] = Uppercase,
                _ => {}
            }
        }
    }
    state.iter().filter(|&&s| s == Uppercase).count() as i32
}

fn main() {
    fn test(func: fn(word: String) -> i32) {
        assert_eq!(func(String::from("cCceDC")), 0);
        assert_eq!(func(String::from("aaAbcBC")), 3);
        assert_eq!(func(String::from("abc")), 0);
        assert_eq!(func(String::from("AbBCab")), 0);
    }
    test(number_of_special_chars);
}
