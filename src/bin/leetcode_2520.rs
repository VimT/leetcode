//! 统计能整除数字的位数

pub fn count_digits(num: i32) -> i32 {
    let mut n = num;
    let mut result = 0;
    while n > 0 {
        if num % (n % 10) == 0 {
            result += 1;
        }
        n /= 10;
    }
    result
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(7), 1);
        assert_eq!(func(121), 2);
        assert_eq!(func(1248), 4);
    }
    test(count_digits);
}
