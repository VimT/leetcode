//! 或运算的最小翻转次数

pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut result = 0;
    for i in 0..31 {
        let (a, b, c) = (a >> i & 1, b >> i & 1, c >> i & 1);
        if c == 0 { result += a + b; } else if a == 0 && b == 0 { result += 1; }
    }
    result
}

fn main() {
    fn test(func: fn(a: i32, b: i32, c: i32) -> i32) {
        assert_eq!(func(2, 6, 5), 3);
        assert_eq!(func(4, 2, 7), 1);
        assert_eq!(func(1, 2, 3), 0);
    }
    test(min_flips);
}
