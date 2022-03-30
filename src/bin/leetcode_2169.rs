//! 得到 0 的操作数

pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
    let mut result = 0;
    if num1 > num2 { std::mem::swap(&mut num1, &mut num2); }
    while num1 > 0 && num2 > 0 {
        result += num2 / num1;
        let tmp = num2 % num1;
        num2 = num1;
        num1 = tmp;
    }
    result
}

fn main() {
    assert_eq!(count_operations(2, 3), 3);
    assert_eq!(count_operations(10, 10), 1);
}
