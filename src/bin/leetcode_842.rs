//! 将数组拆分成斐波那契序列

pub fn split_into_fibonacci(num: String) -> Vec<i32> {
    let s = num.as_bytes();
    let len = s.len();
    let to_num = |start, end| -> i32 {
        let mut num = 0;
        for &ch in &s[start..end] {
            num = num * 10 + (ch - b'0') as i32;
        }
        num
    };
    for first in 1..=(len / 2).min(10) {
        for second in 1..=(len / 2).min(10) {
            let mut a = to_num(0, first);
            let mut b = to_num(first, first + second);
            let mut cur = vec![a, b];
            let mut i = first + second;
            while i < len {
                if let Some(c) = a.checked_add(b) {
                    let cs = c.to_string();
                    let cb = cs.as_bytes();
                    if i + cb.len() > len {
                        break;
                    }
                    if cb == &s[i..i + cb.len()] {
                        cur.push(c);
                        a = b;
                        b = c;
                        i += cb.len();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if i == len && cur.len() >= 3 {
                return cur;
            }
            if s[first] == b'0' {
                break;
            }
        }
        if s[0] == b'0' {
            break;
        }
    }
    vec![]
}

fn main() {
    assert_eq!(split_into_fibonacci(String::from("17522")), vec![17, 5, 22]);
    assert_eq!(split_into_fibonacci(String::from("1101111")), vec![11, 0, 11, 11]);
    assert_eq!(split_into_fibonacci(String::from("112358130")), vec![]);
    assert_eq!(split_into_fibonacci(String::from("0123")), vec![]);
}
