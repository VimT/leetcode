//! 第 N 个神奇数字

/// gcd的更简单的写法
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let l = a / gcd(a, b) * b;
    let (a, b, l) = (a as i64, b as i64, l as i64);
    let mut left = 0;
    let mut right = 1e15 as i64;
    while left < right {
        let mid = left + (right - left) / 2;
        if mid / a + mid / b - mid / l < n as i64 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    (left % MOD) as i32
}

fn main() {
    assert_eq!(nth_magical_number(1, 2, 3), 2);
    assert_eq!(nth_magical_number(4, 2, 3), 6);
}
