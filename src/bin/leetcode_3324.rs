//! 出现在屏幕上的字符串序列

pub fn string_sequence(target: String) -> Vec<String> {
    let mut result = vec![];
    let mut cur = vec![];
    for &ch in target.as_bytes() {
        for x in b'a'..=ch {
            cur.push(x);
            result.push(unsafe { String::from_utf8_unchecked(cur.clone()) });
            cur.pop();
        }
        cur.push(ch);
    }
    result
}

fn main() {
    fn test(func: fn(target: String) -> Vec<String>) {
        assert_eq!(func(String::from("abc")), vec!["a", "aa", "ab", "aba", "abb", "abc"]);
        assert_eq!(func(String::from("he")), vec!["a", "b", "c", "d", "e", "f", "g", "h", "ha", "hb", "hc", "hd", "he"]);
    }
    test(string_sequence);
}
