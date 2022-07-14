//! Nim 游戏

pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(4), false);
        assert_eq!(func(1), true);
        assert_eq!(func(2), true);
    }
    test(can_win_nim);
}
