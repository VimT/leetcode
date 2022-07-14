//! 翻转游戏

pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
    let s = current_state.into_bytes();
    let mut result = vec![];
    for i in 1..s.len() {
        if s[i] == s[i - 1] && s[i] == b'+' {
            let mut tmp = s.clone();
            tmp[i] = b'-';
            tmp[i - 1] = b'-';
            result.push(unsafe { String::from_utf8_unchecked(tmp) });
        }
    }
    result
}

fn main() {
    fn test(func: fn(current_state: String) -> Vec<String>) {
        assert_eq!(func(String::from("++++")), vec!["--++", "+--+", "++--"]);
        assert_eq!(func(String::from("+")).is_empty(), true);
    }
    test(generate_possible_next_moves);
}
