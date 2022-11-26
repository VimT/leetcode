//! 最小 XOR

pub fn minimize_xor(mut num1: i32, num2: i32) -> i32 {
    let c1 = num1.count_ones();
    let mut c2 = num2.count_ones();
    while c2 < c1 {
        num1 &= num1 - 1; // 最低的1变成0
        c2 += 1;
    }
    while c2 > c1 {
        num1 |= num1 + 1; // 最低的0变成1
        c2 -= 1;
    }
    num1
}

fn main() {
    fn test(func: fn(num1: i32, num2: i32) -> i32) {
        assert_eq!(func(3, 5), 3);
        assert_eq!(func(1, 12), 3);
        assert_eq!(func(12, 1), 8);
    }
    test(minimize_xor);
}
