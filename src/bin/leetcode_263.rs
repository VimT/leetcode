//! 丑数

pub fn is_ugly(mut num: i32) -> bool {
    while num > 1 {
        if num % 2 == 0 {
            num /= 2;
        } else if num % 3 == 0 {
            num /= 3;
        } else if num % 5 == 0 {
            num /= 5;
        } else {
            return false;
        }
    }
    if num == 1 { return true; }
    false
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(6), true);
        assert_eq!(func(1), true);
        assert_eq!(func(14), false);
    }
    test(is_ugly);
}
