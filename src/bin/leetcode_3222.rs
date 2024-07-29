//! 求出硬币游戏的赢家

pub fn losing_player(mut x: i32, mut y: i32) -> String {
    let mut run = 0;
    while x >= 1 && y >= 4 {
        x -= 1;
        y -= 4;
        run += 1;
    }
    if run & 1 == 1 {
        String::from("Alice")
    } else {
        String::from("Bob")
    }
}

fn main() {
    fn test(func: fn(x: i32, y: i32) -> String) {
        assert_eq!(func(2, 7), String::from("Alice"));
        assert_eq!(func(4, 11), String::from("Bob"));
    }
    test(losing_player);
}
