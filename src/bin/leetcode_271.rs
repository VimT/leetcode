//! 字符串的编码与解码

use std::convert::TryInto;
use leetcode::svec;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut head = vec![];
        let len = strs.len();
        head.extend_from_slice(&len.to_be_bytes());
        for str in strs {
            head.extend_from_slice(&str.len().to_be_bytes());
            head.extend_from_slice(str.as_bytes());
        }
        unsafe { String::from_utf8_unchecked(head) }
    }

    fn decode(&self, s: String) -> Vec<String> {
        let s = s.as_bytes();
        let len = usize::from_be_bytes((&s[0..8]).try_into().unwrap());
        let mut result = Vec::with_capacity(len);
        let mut i = 8;
        for _ in 0..len {
            let slen = usize::from_be_bytes((&s[i..i + 8]).try_into().unwrap());
            result.push(unsafe { String::from_utf8_unchecked(s[i + 8..i + 8 + slen].to_vec()) });
            i += 8 + slen;
        }
        result
    }
}

fn main() {
    let codec = Codec::new();
    let s = svec!["abc", "def", "{}\\."];
    let encoded = codec.encode(s.clone());
    assert_eq!(s, codec.decode(encoded));
}
