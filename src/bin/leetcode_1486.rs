//! 数组异或操作

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut result = 0;
    for i in 0..n {
        result ^= start + 2 * i;
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, start: i32) -> i32) {
        assert_eq!(func(5, 0), 8);
        assert_eq!(func(4, 3), 8);
    }
    test(xor_operation);
}
