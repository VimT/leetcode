//! 第 N 个泰波那契数

pub fn tribonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    if n == 0 { return a; }
    if n == 1 { return b; }
    if n == 2 { return c; }
    for _ in 3..=n {
        let new = a + b + c;
        a = b;
        b = c;
        c = new;
    }
    c
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(4), 4);
        assert_eq!(func(25), 1389537);
    }
    test(tribonacci);
}
