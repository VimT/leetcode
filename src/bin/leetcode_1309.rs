//! 解码字母到整数映射

pub fn freq_alphabets(s: String) -> String {
    let s = s.as_bytes();
    let mut it = s.iter().rev();
    let mut result = vec![];
    while let Some(&ch) = it.next() {
        if ch == b'#' {
            let b = *it.next().unwrap() - b'0';
            let a = *it.next().unwrap() - b'0';
            result.push(a * 10 + b - 1 + b'a');
        } else {
            result.push(ch - b'1' + b'a');
        }
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("10#11#12")), String::from("jkab"));
        assert_eq!(func(String::from("1326#")), String::from("acz"));
    }
    test(freq_alphabets);
}
