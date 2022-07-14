//! 快乐数

pub fn is_happy(mut n: i32) -> bool {
    for _ in 0..1000 {
        let mut tmp = 0;
        while n > 0 {
            let p = n % 10;
            tmp += p * p;
            n /= 10;
        }
        if tmp == 1 { return true; }
        n = tmp;
    }
    false
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(19), true);
        assert_eq!(func(2), false);
    }
    test(is_happy);
}
