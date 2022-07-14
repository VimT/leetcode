//! 个位数字为 K 的整数之和

pub fn minimum_numbers(num: i32, k: i32) -> i32 {
    if num == 0 { return 0; }
    for i in 1..=10 {
        if k * i % 10 == num % 10 && k * i <= num {
            return i;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(num: i32, k: i32) -> i32) {
        assert_eq!(func(10, 1), 10);
        assert_eq!(func(10, 8), -1);
        assert_eq!(func(58, 9), 2);
        assert_eq!(func(37, 2), -1);
        assert_eq!(func(0, 7), 0);
    }
    test(minimum_numbers);
}
