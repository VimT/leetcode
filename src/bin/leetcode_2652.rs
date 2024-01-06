//! 倍数求和

/// 容斥原理
pub fn sum_of_multiples(n: i32) -> i32 {
    let s = |m: i32| -> i32 {
        return (1 + n / m) * (n / m) / 2 * m;
    };
    s(3) + s(5) + s(7) - s(15) - s(21) - s(35) + s(105)
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(7), 21);
        assert_eq!(func(10), 40);
        assert_eq!(func(9), 30);
    }
    test(sum_of_multiples);
}
