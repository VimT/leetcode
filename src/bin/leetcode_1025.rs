//! 除数博弈

pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    assert_eq!(divisor_game(2), true);
    assert_eq!(divisor_game(3), false);
}
