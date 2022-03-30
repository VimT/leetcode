//! 字符串相乘

fn add(mut num1: &[u8], mut num2: &[u8]) -> Vec<u8> {
    let pos = match (num1.first(), num2.first()) {
        (Some(b'-'), Some(b'-')) => {
            num1 = &num1[1..];
            num2 = &num2[1..];
            false
        }
        (_, Some(b'-')) => {
            return minus(num1, &num2[1..]);
        }
        (Some(b'-'), _) => {
            return minus(num2, &num1[1..]);
        }
        _ => true
    };
    let mut result = vec![];
    let mut carry = 0;
    let mut it1 = num1.iter().rev();
    let mut it2 = num2.iter().rev();
    loop {
        let num = match (it1.next(), it2.next()) {
            (Some(a), Some(b)) => { (*a - b'0') + (*b - b'0') }
            (None, Some(b)) => { *b - b'0' }
            (Some(a), None) => { *a - b'0' }
            (None, None) => {
                break;
            }
        } + carry;
        carry = num / 10;
        result.push(num % 10 + b'0');
    }
    if carry != 0 {
        result.push(carry + b'0');
    }
    if !pos {
        result.push(b'-');
    }
    result.reverse();
    result
}

fn minus<'a>(mut num1: &'a [u8], mut num2: &'a [u8]) -> Vec<u8> {
    if num1 == num2 { return vec![b'0']; }
    let mut positive = true;
    if num1.len() < num2.len() || (num1.len() == num2.len() && num1 < num2) {
        positive = false;
        let tmp = num1;
        num1 = num2;
        num2 = tmp;
    }
    let mut result = vec![];
    let mut carry = 0;
    let mut it1 = num1.iter().rev();
    let mut it2 = num2.iter().rev();
    loop {
        let mut num = match (it1.next(), it2.next()) {
            (Some(a), Some(b)) => { *a as i8 - *b as i8 }
            (None, Some(b)) => { -((*b - b'0') as i8) }
            (Some(a), None) => { (*a - b'0') as i8 }
            (None, None) => {
                break;
            }
        } + carry;
        carry = 0;
        if num < 0 {
            num += 10;
            carry = -1;
        }
        result.push(num as u8 % 10 + b'0');
    }
    while !result.is_empty() {
        if *result.last().unwrap() == b'0' {
            result.pop();
        } else {
            break;
        }
    }
    if !positive {
        result.push(b'-');
    }
    result.reverse();
    result
}

fn shift(num: &[u8], len: usize) -> Vec<u8> {
    let mut result = num.to_vec();
    if num == b"0" {
        return result;
    }
    for _ in 0..len {
        result.push(b'0');
    }
    result
}

fn multi_one(num: &[u8], multi: u8) -> Vec<u8> {
    let mut carry = 0;
    let mut result = Vec::with_capacity(num.len());
    for &i in num.iter().rev() {
        let new = carry + (i - b'0') * multi;
        result.push(new % 10 + b'0');
        carry = new / 10;
    }
    if carry != 0 {
        result.push(carry + b'0');
    }
    result.reverse();
    result
}

pub fn make_len(num: &[u8], len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);
    for _ in 0..len - num.len() {
        result.push(b'0');
    }
    result.extend_from_slice(num);
    result
}

/// 小学生乘法
pub fn multiply(num1: String, num2: String) -> String {
    let b1 = num1.as_bytes();
    let b2 = num2.as_bytes();
    let mut result = vec![b'0'];

    for (idx, &num) in b2.iter().rev().enumerate() {
        result = add(&result, &shift(&multi_one(b1, num - b'0'), idx));
    }
    unsafe { String::from_utf8_unchecked(result) }
}

/// 小学生乘法改进，不使用进位
pub fn multiply_optimise(num1: String, num2: String) -> String {
    let mut ans = vec![0; num1.len() + num2.len()];
    let b1: Vec<u8> = num1.bytes().map(|x| x - b'0').collect();
    let b2: Vec<u8> = num2.bytes().map(|x| x - b'0').collect();
    // i位与j位相乘，结果保存在i+j位上
    for i in 0..b1.len() {
        for j in 0..b2.len() {
            ans[i + j + 1] += (b1[i] * b2[j]) as u32;
        }
    }

    // 处理进位
    for i in (0..ans.len()).rev() {
        if ans[i] >= 10 {
            ans[i - 1] += ans[i] / 10;
            ans[i] %= 10;
        }
    }

    // 去掉前面的0
    let mut i = 0;
    while i < ans.len() - 1 && ans[i] == 0 { i += 1; }

    unsafe { String::from_utf8_unchecked(ans[i..].iter().map(|&x| x as u8 + b'0').collect()) }
}

/// Karatsuba算法
/// 5678 * 1234的过程，首先是拆分成 56 * 10^2 + 78 and 12 * 10^2 + 34 四个部分
/// 然后分别计算ac, bd, (a+b)*(c+d)
/// 最后再用第三个算式的结果减去前面两个（其实得到的就是bc+ad，但是减少了乘法步骤）
/// 然后，计算式1后面加4个0，计算式2后面不加，计算式3后面加2个0，再把这三者相加，就是正确结果。
pub fn multiply_karatsuba(num1: String, num2: String) -> String {
    fn karatsuba(num1: &[u8], num2: &[u8]) -> Vec<u8> {
        if num1.is_empty() || num2.is_empty() { return vec![b'0']; }
        if num1.len() == 1 && num2.len() == 1 {
            return ((num1[0] - b'0') as i32 * (num2[0] - b'0') as i32).to_string().into_bytes();
        }
        let len = num1.len().max(num2.len());
        let num1 = make_len(num1, len);
        let num2 = make_len(num2, len);
        let mid = len / 2;
        let m = len - mid;
        let (x1, x0) = num1.split_at(mid);
        let (y1, y0) = num2.split_at(mid);
        let z0 = karatsuba(x0, y0);
        let z1 = karatsuba(&add(x1, x0), &add(y1, y0));
        let z2 = karatsuba(x1, y1);
        let r1 = shift(&z2, 2 * m);
        let r2 = shift(&minus(&minus(&z1, &z2), &z0), m);
        return add(&add(&r1, &r2), &z0);
    }
    unsafe { String::from_utf8_unchecked(karatsuba(num1.as_bytes(), num2.as_bytes())) }
}

fn main() {
    assert_eq!(multiply_karatsuba(String::from("123"), String::from("3")), "369");
    fn test(func: fn(num1: String, num2: String) -> String) {
        assert_eq!(func(String::from("2"), String::from("3")), String::from("6"));
        assert_eq!(func(String::from("123"), String::from("456")), String::from("56088"));
    }
    assert_eq!(add(b"-123", b"-3"), b"-126");
    assert_eq!(add(b"123", b"-3"), b"120");
    assert_eq!(add(b"123", b"456"), b"579");
    assert_eq!(add(b"123", b"1"), b"124");
    assert_eq!(minus(b"10", b"6"), b"4");
    assert_eq!(minus(b"465", b"123"), b"342");
    assert_eq!(shift(b"123", 1), b"1230");
    assert_eq!(shift(b"0", 1), b"0");
    test(multiply_optimise);
    test(multiply);
    test(multiply_karatsuba);
}
