//! 神奇字符串

pub fn magical_string(n: i32) -> i32 {
    if n <= 3 { return 1; }
    let n = n as usize;
    let mut str = Vec::with_capacity(n + 10);
    str.push(b'1');
    str.push(b'2');
    str.push(b'2');
    let mut result = 1;
    let mut i = 2;
    while str.len() < n {
        let &last_ch = str.last().unwrap();
        if str[i] == b'1' {
            if last_ch == b'1' {
                str.push(b'2');
            } else {
                str.push(b'1');
                result += 1;
            }
        } else {
            if last_ch == b'1' {
                str.push(b'2');
                str.push(b'2');
            } else {
                str.push(b'1');
                str.push(b'1');
                result += if str.len() <= n { 2 } else { 1 };
            }
        }
        i += 1;
    }
    result
}

fn main() {
    assert_eq!(magical_string(5), 3);
    assert_eq!(magical_string(4), 2);
    assert_eq!(magical_string(6), 3);
    assert_eq!(magical_string(1), 1);
}
