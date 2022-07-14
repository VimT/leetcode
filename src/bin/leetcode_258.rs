//! 各位相加

pub fn add_digits(num: i32) -> i32 {
    (num - 1) % 9 + 1
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(38), 2);
        assert_eq!(func(0), 0);
    }
    test(add_digits);
}
