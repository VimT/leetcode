//! 重新排列单词间的空格

pub fn reorder_spaces(text: String) -> String {
    let len = text.len();
    let words: Vec<&str> = text.split_ascii_whitespace().collect();
    let space_len = len - words.iter().map(|x| x.len()).sum::<usize>();
    let wlen = words.len();
    let average = if wlen == 1 { space_len } else { space_len / (wlen - 1) };
    let mut result = Vec::with_capacity(len);
    for word in words {
        result.extend_from_slice(word.as_bytes());
        for _ in 0..average { if result.len() < len { result.push(b' '); } }
    }
    while result.len() < len { result.push(b' '); }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(text: String) -> String) {
        assert_eq!(func(String::from("a b   c d")), String::from("a b c d  "));
        assert_eq!(func(String::from("  this   is  a sentence ")), String::from("this   is   a   sentence"));
        assert_eq!(func(String::from(" practice   makes   perfect")), String::from("practice   makes   perfect "));
        assert_eq!(func(String::from("hello   world")), String::from("hello   world"));
        assert_eq!(func(String::from("  walks  udp package   into  bar a")), String::from("walks  udp  package  into  bar  a "));
        assert_eq!(func(String::from("a")), String::from("a"));
    }
    test(reorder_spaces);
}
