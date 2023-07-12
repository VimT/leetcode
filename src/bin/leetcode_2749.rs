//! 得到整数零需要执行的最少操作数

pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    let num1 = num1 as i64;
    let num2 = num2 as i64;
    for i in 1..64 {
        let num = num1 - i * num2;
        if num < i { break; }
        if num.count_ones() <= i as u32 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(num1: i32, num2: i32) -> i32) {
        assert_eq!(func(3, -2), 3);
        assert_eq!(func(5, 7), -1);
    }
    test(make_the_integer_zero);
}
