//! 翻转游戏 II

pub fn can_win(current_state: String) -> bool {
    fn dfs(s: &mut Vec<u8>) -> bool {
        for i in 1..s.len() {
            if s[i] == s[i - 1] && s[i] == b'+' {
                s[i - 1] = b'-';
                s[i] = b'-';
                let win = !dfs(s);
                s[i - 1] = b'+';
                s[i] = b'+';
                if win { return true; }
            }
        }
        false
    }
    let mut s = current_state.into_bytes();
    dfs(&mut s)
}

fn main() {
    fn test(func: fn(current_state: String) -> bool) {
        assert_eq!(func(String::from("+++++")), false);
        assert_eq!(func(String::from("++++")), true);
        assert_eq!(func(String::from("+")), false);
    }
    test(can_win);
}
