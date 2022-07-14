//! 三元表达式解析器

pub fn parse_ternary(expression: String) -> String {
    fn parse<'a>(e: &'a [u8], i: &mut usize) -> &'a [u8] {
        return if matches!(e[*i], b'T' | b'F') && *i + 1 < e.len() && e[*i + 1] == b'?' {
            let is_true = e[*i] == b'T';
            *i += 2;
            let left = parse(e, i);
            assert_eq!(e[*i], b':');
            *i += 1;
            let right = parse(e, i);
            if is_true { left } else { right }
        } else {
            if !matches!(e[*i], b'0'..=b'9'|b'T'|b'F') {
                unreachable!();
            }
            *i += 1;
            &e[*i - 1..*i]
        };
    }
    unsafe { String::from_utf8_unchecked(parse(expression.as_bytes(), &mut 0).to_vec()) }
}

fn main() {
    fn test(func: fn(expression: String) -> String) {
        assert_eq!(func(String::from("T?2:3")), String::from("2"));
        assert_eq!(func(String::from("F?1:T?4:5")), String::from("4"));
        assert_eq!(func(String::from("T?T?F:5:3")), String::from("F"));
    }
    test(parse_ternary);
}
