//! 可被 K 整除的最小整数

/// 1111 = 111 * 10 + 1
pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 2 == 0 || k % 5 == 0 { return -1; }
    let mut result = 1;
    let mut n = 1;
    while n % k != 0 {
        n %= k;
        n = n * 10 + 1;
        result += 1;
    }
    result
}

fn main() {
    assert_eq!(smallest_repunit_div_by_k(1), 1);
    assert_eq!(smallest_repunit_div_by_k(2), -1);
    assert_eq!(smallest_repunit_div_by_k(3), 3);
}
