//! TinyURL 的加密与解密

use std::collections::HashMap;

use rand::{RngCore, thread_rng};

fn base62a2i(s: &str) -> u64 {
    let mut result = 0;
    for &ch in s.as_bytes() {
        if ch >= b'0' && ch <= b'9' {
            result = result * 62 + (ch - b'0') as u64;
        } else if ch >= b'a' && ch <= b'z' {
            result = result * 62 + (ch - b'a' + 10) as u64;
        } else if ch >= b'A' && ch <= b'Z' {
            result = result * 62 + (ch - b'A' + 36) as u64;
        } else {
            unreachable!()
        }
    }
    result
}

fn base62i2a(mut i: u64) -> String {
    let mut result = vec![];
    while i > 0 {
        let yu = (i % 62) as u8;
        let char = if yu < 10 {
            b'0' + yu
        } else if yu >= 10 && yu < 36 {
            b'a' + (yu - 10)
        } else if yu >= 36 && yu < 62 {
            b'A' + (yu - 36)
        } else {
            unreachable!()
        };
        result.push(char);
        i /= 62;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

struct Codec {
    s2l: HashMap<u64, String>,
    l2s: HashMap<String, u64>,
}

impl Codec {
    fn new() -> Self {
        Self { s2l: HashMap::new(), l2s: HashMap::new() }
    }

    fn format_url(short: u64) -> String {
        format!("http://tinyurl.com/{}", base62i2a(short))
    }

    fn short_url_extra(short_url: String) -> u64 {
        base62a2i(short_url.rsplit("/").next().unwrap())
    }

    fn encode(&mut self, long_url: String) -> String {
        if let Some(short) = self.l2s.get(&long_url) {
            return Self::format_url(*short);
        }
        let mut rng = thread_rng();
        let mut random = rng.next_u64();
        while self.s2l.contains_key(&random) {
            random = rng.next_u64();
        }
        self.s2l.insert(random, long_url.clone());
        self.l2s.insert(long_url, random);
        Self::format_url(random)
    }

    fn decode(&self, short_url: String) -> String {
        if let Some(long) = self.s2l.get(&Self::short_url_extra(short_url)) {
            return long.clone();
        }
        String::new()
    }
}

fn main() {
    let mut codec = Codec::new();
    let url = String::from("https://leetcode.com/problems/design-tinyurl");
    let encode = codec.encode(url.clone());
    println!("{}", encode);
    assert_eq!(codec.decode(encode), url);
}
