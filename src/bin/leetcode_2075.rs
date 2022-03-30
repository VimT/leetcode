//! 解码斜向换位密码

pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let rows = rows as usize;
    let s = encoded_text.as_bytes();
    let mut result = vec![];
    let cols = s.len() / rows;
    for j in 0..cols {
        for i in 0..rows {
            let idx = i * cols + j + i;
            if idx >= s.len() { break; }
            result.push(s[idx]);
        }
    }
    while !result.is_empty() && *result.last().unwrap() == b' ' {
        result.pop();
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(decode_ciphertext(String::from("ch   ie   pr"), 3), "cipher");
    assert_eq!(decode_ciphertext(String::from("iveo    eed   l te   olc"), 4), "i love leetcode");
    assert_eq!(decode_ciphertext(String::from("coding"), 1), "coding");
    assert_eq!(decode_ciphertext(String::from(" b  ac"), 2), " abc");
}

