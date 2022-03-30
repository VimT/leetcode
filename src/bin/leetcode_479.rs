//! 最大回文数乘积

const MOD: i64 = 1337;

pub fn largest_palindrome(n: i32) -> i32 {
    if n == 1 { return 9; }
    let m = 10_i64.pow(n as u32);
    for x in (2..m).step_by(2) {
        let upper = m - x;
        let mut lower = 0;
        let mut num = upper;
        while num > 0 {
            lower = lower * 10 + num % 10;
            num /= 10;
        }
        let tmp = x * x - 4 * lower;
        if tmp < 0 { continue; }
        let sq = (tmp as f64).sqrt() as i64;
        if sq * sq == tmp {
            return ((upper * m + lower) % MOD) as i32;
        }
    }
    -1
}

fn main() {
    assert_eq!(largest_palindrome(8), 475);
    assert_eq!(largest_palindrome(4), 597);
    assert_eq!(largest_palindrome(3), 123);
    assert_eq!(largest_palindrome(2), 987);
    assert_eq!(largest_palindrome(1), 9);
}
