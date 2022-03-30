//! 阶乘后的零

pub fn trailing_zeroes(mut n: i32) -> i32 {
    let mut ans = 0;
    while n >= 5 {
        n = n / 5;
        ans += n;
    }
    ans
}


fn main() {
    assert_eq!(trailing_zeroes(3), 0);
    assert_eq!(trailing_zeroes(5), 1);
    assert_eq!(trailing_zeroes(0), 0);
}
