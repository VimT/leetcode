//! 找出中枢整数

pub fn pivot_integer(n: i32) -> i32 {
    let x = (((n * n + n) / 2) as f64).sqrt() as i32;
    if 2 * (x * x) == n * n + n {
        return x;
    }
    -1
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(8), 6);
        assert_eq!(func(1), 1);
        assert_eq!(func(4), -1);
    }
    test(pivot_integer);
}
