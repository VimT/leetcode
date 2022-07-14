//! 移除 9

/// 第 n 个 9 进制数字。
pub fn new_integer(mut n: i32) -> i32 {
    let mut result = vec![];
    while n > 0 {
        result.push(n % 9);
        n /= 9;
    }
    let mut num = 0;
    for &i in result.iter().rev() {
        num = num * 10 + i;
    }
    num
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(9), 10);
        assert_eq!(func(10), 11);
    }
    test(new_integer);
}
