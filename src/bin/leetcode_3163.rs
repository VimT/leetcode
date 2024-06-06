//! 压缩字符串 III

pub fn compressed_string(word: String) -> String {
    let mut result = vec![];
    let word = word.into_bytes();
    let mut i = 0;
    let len = word.len();
    while i < len {
        let ch = word[i];
        let mut cnt = 1;
        i += 1;
        while i < len && cnt < 9 && word[i] == ch {
            i += 1;
            cnt += 1;
        }
        result.push(b'0' + cnt);
        result.push(ch);
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    fn test(func: fn(word: String) -> String) {
        assert_eq!(func(String::from("abcde")), String::from("1a1b1c1d1e"));
        assert_eq!(func(String::from("aaaaaaaaaaaaaabb")), String::from("9a5a2b"));
    }
    test(compressed_string);
}
