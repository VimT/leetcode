//! 交替数字和

pub fn alternate_digit_sum(mut n: i32) -> i32 {
    let mut v = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.reverse();
    let mut result = 0;
    let mut t = 1;
    for wei in v {
        result += wei * t;
        t *= -1;
    }
    result
}

pub fn alternate_digit_sum2(mut n: i32) -> i32 {
    let mut result = 0;
    let mut sign = 1;
    while n > 0 {
        result += n % 10 * sign;
        n /= 10;
        sign = -sign;
    }
    result * -sign
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(521), 4);
        assert_eq!(func(111), 1);
        assert_eq!(func(886996), 0);
    }
    test(alternate_digit_sum);
    test(alternate_digit_sum2);
}
