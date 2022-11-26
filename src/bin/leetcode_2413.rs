//! 最小偶倍数

pub fn smallest_even_multiple(n: i32) -> i32 {
    if n & 1 == 1 { n * 2 } else { n }
}


fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(5), 10);
        assert_eq!(func(6), 6);
    }
    test(smallest_even_multiple);
}
