//! 哈沙德数

pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut y = 0;
    let mut num = x;
    while num > 0 {
        y += num % 10;
        num /= 10;
    }
    if x % y == 0 { y } else { -1 }
}

fn main() {
    fn test(func: fn(x: i32) -> i32) {
        assert_eq!(func(18), 9);
        assert_eq!(func(23), -1);
    }
    test(sum_of_the_digits_of_harshad_number);
}
