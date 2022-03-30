//! 索引处的解码字符串

pub fn decode_at_index(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut cur_len = 0;
    for i in 0..len {
        if s[i].is_ascii_digit() {
            cur_len *= (s[i] - b'0') as i64;
        } else {
            cur_len += 1;
        }
    }
    let mut k = k as i64;
    for i in (0..len).rev() {
        k %= cur_len;
        if k == 0 && s[i].is_ascii_lowercase() {
            return (s[i] as char).to_string();
        }
        if s[i].is_ascii_digit() {
            cur_len /= (s[i] - b'0') as i64;
        } else {
            cur_len -= 1;
        }
    }
    String::new()
}

fn main() {
    assert_eq!(decode_at_index(String::from("leet2code3"), 10), String::from("o"));
    assert_eq!(decode_at_index(String::from("ha22"), 5), String::from("h"));
    assert_eq!(decode_at_index(String::from("a2345678999999999999999"), 1), String::from("a"));
}
