//! 斐波那契数

pub fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    if n == 0 { return 0; }
    for _ in 0..n - 1 {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(2), 1);
        assert_eq!(func(3), 2);
        assert_eq!(func(4), 3);
    }
    test(fib);
}
