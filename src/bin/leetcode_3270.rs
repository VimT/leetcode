//! 求出数字答案

pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
    let mut result = 0;
    let mut pow10 = 1;
    while num1 > 0 && num2 > 0 && num3 > 0 {
        result += (num1 % 10).min((num2 % 10).min(num3 % 10)) * pow10;
        num1 /= 10;
        num2 /= 10;
        num3 /= 10;
        pow10 *= 10;
    }
    result
}

fn main() {
    fn test(func: fn(num1: i32, num2: i32, num3: i32) -> i32) {
        assert_eq!(func(1, 10, 1000), 0);
        assert_eq!(func(987, 879, 798), 777);
        assert_eq!(func(1, 2, 3), 1);
    }
    test(generate_key);
}
