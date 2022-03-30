//! 两数相除

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    /// 11 / 3 => 3 < 11,count+1 => 6 < 11, count+1 => 12 > 11 (count = 2) => 11-6 = 5 / 3 => ..
    fn div(a: i64, b: i64) -> i64 {
        if a < b { return 0; }
        let mut count = 1;
        let mut tb = b;
        while tb + tb <= a {
            count += count;
            tb += tb;
        }
        return count + div(a - tb, b);
    }
    if dividend == 0 { return 0; }
    if divisor == 1 { return dividend; }
    if divisor == -1 {
        if dividend > i32::MIN {
            return -dividend;
        }
        return i32::MAX;
    }

    let mut a = dividend as i64;
    let mut b = divisor as i64;
    let mut sign = 1;
    if (a > 0 && b < 0) || (a < 0 && b > 0) {
        sign = -1;
    }
    a = a.abs();
    b = b.abs();
    let ans = div(a, b);
    return if sign == 1 {
        if ans > i32::MAX as i64 { i32::MAX } else { ans as i32 }
    } else { -ans as i32 };
}

fn main() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
}
