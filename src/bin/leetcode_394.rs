//! 字符串解码

pub fn decode_string(s: String) -> String {
    fn inner(b: &[u8], i: &mut usize) -> Vec<u8> {
        let mut ans = vec![];
        let mut num = 0;
        while *i < b.len() {
            let c = b[*i];
            if c.is_ascii_digit() {
                num = 10 * num + (c - b'0') as usize;
            } else if c == b'[' {
                *i += 1;
                let tmp = inner(b, i);
                ans.append(&mut tmp.repeat(num));
                num = 0;
            } else if c == b']' {
                break;
            } else {
                ans.push(c)
            }
            *i += 1;
        }
        ans
    }
    let mut i = 0;
    let ans = inner(s.as_bytes(), &mut i);
    unsafe { String::from_utf8_unchecked(ans) }
}


fn main() {
    assert_eq!(decode_string(String::from("3[a]2[bc]")), String::from("aaabcbc"));
    assert_eq!(decode_string(String::from("3[a2[c]]")), String::from("accaccacc"));
    assert_eq!(decode_string(String::from("2[abc]3[cd]ef")), String::from("abcabccdcdcdef"));
}
