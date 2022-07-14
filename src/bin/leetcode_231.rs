//! 2 的幂

pub fn is_power_of_two(mut n: i32) -> bool {
    if n <= 0 { return false; }
    if n > 0 {
        n &= n - 1;
        if n > 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(1), true);
        assert_eq!(func(16), true);
        assert_eq!(func(3), false);
    }
    test(is_power_of_two);
}
