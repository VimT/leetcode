//! 数字 1 的个数

pub fn count_digit_one(n: i32) -> i32 {
    let n = n as i64;
    let mut ans = 0;
    let mut div = 1;
    while n / div > 0 {
        let left = n / div / 10;
        let right = n % div;
        let num = n / div % 10;
        if num == 0 {
            ans += left * div;
        } else if num == 1 {
            ans += left * div + right + 1;
        } else {
            ans += (left + 1) * div;
        }
        div *= 10;
    }
    ans as i32
}

fn main() {
    assert_eq!(count_digit_one(13), 6);
    assert_eq!(count_digit_one(0), 0);
}
