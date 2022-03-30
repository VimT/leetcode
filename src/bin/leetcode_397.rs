//! 整数替换

pub fn integer_replacement(n: i32) -> i32 {
    let mut n = n as i64;
    let mut result = 0;
    while n > 1 {
        if n == 3 {
            result += 2;
            break;
        }
        if n & 1 == 1 {
            if n.trailing_ones() > 1 {
                n += 1;
                result += 1;
            } else {
                n -= 1;
                result += 1;
            }
        } else {
            let zero = n.trailing_zeros();
            n >>= zero;
            result += zero as i32;
        }
    }
    result
}

fn main() {
    assert_eq!(integer_replacement(2147483647), 32);
    assert_eq!(integer_replacement(6), 3);
    assert_eq!(integer_replacement(2), 1);
    assert_eq!(integer_replacement(1), 0);
    assert_eq!(integer_replacement(3), 2);
    assert_eq!(integer_replacement(8), 3);
    assert_eq!(integer_replacement(7), 4);
    assert_eq!(integer_replacement(4), 2);
}
