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

/// 链表环检测，快慢指针
pub fn is_happy2(n: i32) -> bool {
    fn next(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            let p = n % 10;
            result += p * p;
            n /= 10;
        }
        result
    }
    let mut p1 = n;
    let mut p2 = n;
    loop {
        p1 = next(p1);
        p2 = next(next(p2));
        if p1 == p2 {
            return p1 == 1
        }
    }
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(19), true);
        assert_eq!(func(2), false);
    }
    test(is_happy);
    test(is_happy2);
}
