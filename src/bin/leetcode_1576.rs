//! 替换所有的问号

pub fn modify_string(s: String) -> String {
    let mut s = s.into_bytes();
    let len = s.len();
    for i in 0..len {
        if s[i] == b'?' {
            for choose in 0 + b'a'..=26 + b'a' {
                if (i > 0 && s[i - 1] == choose) || (i + 1 < len && s[i + 1] == choose) {
                    continue;
                }
                s[i] = choose;
                break;
            }
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(modify_string(String::from("?zs")), String::from("azs"));
    assert_eq!(modify_string(String::from("ubv?w")), String::from("ubvaw"));
}
