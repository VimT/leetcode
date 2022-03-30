//! 密钥格式化

pub fn license_key_formatting(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = Vec::with_capacity(len);
    let mut h = 0;
    for &ch in s {
        if ch == b'-' { h += 1; }
    }
    let k = k as usize;
    let mut first = (len - h) % k;
    let mut i = 0;
    while first > 0 {
        if s[i] != b'-' {
            result.push(s[i].to_ascii_uppercase());
            first -= 1;
        }
        i += 1;
    }

    for _ in 0..(len - h) / k {
        if !result.is_empty() { result.push(b'-'); }
        let mut k = k;
        while k > 0 {
            if s[i] != b'-' {
                result.push(s[i].to_ascii_uppercase());
                k -= 1;
            }
            i += 1;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(license_key_formatting(String::from("5F3Z-2e-9-w"), 4), String::from("5F3Z-2E9W"));
    assert_eq!(license_key_formatting(String::from("2-5g-3-J"), 2), String::from("2-5G-3J"));
}
