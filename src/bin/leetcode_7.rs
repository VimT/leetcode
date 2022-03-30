//! 整数反转

pub fn reverse(mut x: i32) -> i32 {
    let mut ans: i32 = 0;
    while x != 0 {
        let tmp = x % 10;
        x = x / 10;
        match ans.checked_mul(10).and_then(|v| v.checked_add(tmp)) {
            Some(v) => ans = v,
            None => return 0
        }
    }
    ans
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}
