//! 向表达式添加括号后的最小结果

pub fn minimize_result(expression: String) -> String {
    let s = expression.as_bytes();
    let mut left = 0;
    let mut i = 0;
    while i < s.len() && s[i] != b'+' {
        left = left * 10 + (s[i] - b'0') as i32;
        i += 1;
    }
    let left_len = i;
    i += 1;
    let right_len = s.len() - i;
    let mut right = 0;
    while i < s.len() {
        right = right * 10 + (s[i] - b'0') as i32;
        i += 1;
    }
    let mut min = left + right;
    let mut ij = (0, 0);
    for i in 0..left_len {
        for j in 0..right_len {
            let l1 = if i == 0 { 1 } else { left / 10i32.pow((left_len - i) as u32) };
            let l2 = left % 10i32.pow((left_len - i) as u32);
            let r1 = right / 10i32.pow(j as u32);
            let r2 = if j == 0 { 1 } else { right % 10i32.pow(j as u32) };
            if l1 * (l2 + r1) * r2 < min {
                min = l1 * (l2 + r1) * r2;
                ij = (i, j);
            }
        }
    }
    let mut s = s.to_vec();
    s.insert(left_len + (right_len - ij.1) + 1, b')');
    s.insert(ij.0, b'(');
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    fn test(func: fn(expression: String) -> String) {
        assert_eq!(func(String::from("247+38")), String::from("2(47+38)"));
        assert_eq!(func(String::from("12+34")), String::from("1(2+3)4"));
        assert_eq!(func(String::from("999+999")), String::from("(999+999)"));
    }
    test(minimize_result);
}
