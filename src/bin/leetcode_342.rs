//! 4的幂

pub fn is_power_of_four(n: i32) -> bool {
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(16), true);
        assert_eq!(func(5), false);
        assert_eq!(func(1), true);
    }
    test(is_power_of_four);
}
