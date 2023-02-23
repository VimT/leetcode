//! 将整数减少到零需要的最少操作数

pub fn min_operations(mut n: i32) -> i32 {
    let mut result = 0;
    while n != 0 {
        n >>= n.trailing_zeros();
        if n.trailing_ones() > 1 {
            n += 1;
            result += 1;
        } else {
            n >>= 1;
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(39), 3);
        assert_eq!(func(54), 3);
    }
    test(min_operations);
}
