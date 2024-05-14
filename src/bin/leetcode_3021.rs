//! Alice 和 Bob 玩鲜花游戏

pub fn flower_game(n: i32, m: i32) -> i64 {
    (n as i64) * (m as i64) / 2
}

fn main() {
    fn test(func: fn(n: i32, m: i32) -> i64) {
        assert_eq!(func(3, 2), 3);
        assert_eq!(func(1, 1), 0);
    }
    test(flower_game);
}
