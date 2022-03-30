//! 计算各个位数不同的数字个数

/// dp[i] = dp[i - 1] * (11 - i)
/// dp[i] 为i位数中各个位数不同的数字个数
pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    if n == 0 { return 1; }
    let mut a = 1;
    let mut b = 9;
    for i in 2..=n {
        a += b;
        b *= 11 - i;
    }
    b + a
}

fn main() {
    assert_eq!(count_numbers_with_unique_digits(2), 91);
    assert_eq!(count_numbers_with_unique_digits(0), 1);
}
