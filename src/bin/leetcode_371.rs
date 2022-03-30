//! 两整数之和


pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    let mut wei = 0;
    let mut carry = 0;
    let mut result = 0;
    while (a != 0 || b != 0) && wei < 32 {
        match (a & 1, b & 1, carry) {
            (0, 0, 0) => {}
            (0, 0, 1) | (0, 1, 0) | (1, 0, 0) => {
                result |= 1 << wei;
                carry = 0;
            }
            (1, 1, 0) | (0, 1, 1) | (1, 0, 1) => { carry = 1; }
            (1, 1, 1) => {
                carry = 1;
                result |= 1 << wei;
            }
            (_, _, _) => { panic!() }
        }
        wei += 1;
        a >>= 1;
        b >>= 1;
    }
    if carry == 1 && wei < 32 {
        result |= 1 << wei;
    }
    result
}

pub fn get_sum_correct(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}

fn main() {
    fn test(func: fn(a: i32, b: i32) -> i32) {
        assert_eq!(func(10, 11), 21);
        assert_eq!(func(-1000, -1), -1001);
        assert_eq!(func(-2, -3), -5);
        assert_eq!(func(-1, 1), 0);
        assert_eq!(func(2, 3), 5);
        assert_eq!(func(1, 2), 3);
    }
    test(get_sum);
    test(get_sum_correct);
}
