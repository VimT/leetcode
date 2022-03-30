//! 负二进制转换

/// 需要分正数位和复数位（也就是从右往左的偶数下标和奇数下标），
/// 假如是正数位，那么mod2的余数就是这一位的值，
/// 假如是负数位，那么mod2为0的时候正常，假如mod2之后是1
/// 注意此时这一位可以是1，那么整个数就少了1，需要在原数字上再加1，才能补回来
pub fn base_neg2(mut n: i32) -> String {
    if n == 0 { return String::from("0"); }
    let mut result = vec![];
    let mut i = false;
    while n != 0 {
        result.push((n & 1) as u8 + b'0');
        if i {
            n = (n >> 1) + (n & 1);
        } else {
            n >>= 1;
        }
        i = !i;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}


pub fn base_neg2_another(mut n: i32) -> String {
    if n == 0 { return String::from("0"); }
    let mut result = vec![];
    while n != 0 {
        result.push((n & 1) as u8 + b'0');
        n = -(n >> 1);
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(base_neg2_another(2), "110");
    assert_eq!(base_neg2_another(3), "111");
    assert_eq!(base_neg2_another(4), "100");
}
