//! 最长的美好子字符串

pub fn longest_nice_substring(oris: String) -> String {
    let s = oris.as_bytes();
    let len = s.len();
    let mut max = 0;
    let mut start = 0;
    for i in 0..len {
        let mut cur = 0u64;
        for j in i..len {
            if s[j].is_ascii_lowercase() {
                cur |= 1 << (s[j] - b'a');
            } else {
                cur |= 1 << (s[j] - b'A' + 26);
            }
            if cur & ((1 << 26) - 1) == cur >> 26 {
                if j - i + 1 > max {
                    max = j - i + 1;
                    start = i;
                }
            }
        }
    }
    oris[start..start + max].to_string()
}

fn main() {
    assert_eq!(longest_nice_substring(String::from("YazaAay")), String::from("aAa"));
    assert_eq!(longest_nice_substring(String::from("Bb")), String::from("Bb"));
    assert_eq!(longest_nice_substring(String::from("c")), String::from(""));
}
