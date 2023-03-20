//! HTML 实体解析器

pub fn entity_parser(mut text: String) -> String {
    for (s, t) in [
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&frasl;", "/"),
        ("&amp;", "&"),
    ] {
        text = text.replace(s, t)
    }
    text
}

/// 想象成栈
pub fn entity_parser2(text: String) -> String {
    let mut s = text.into_bytes();
    let len = s.len();
    let mut i = 0;
    let mut j = 0;
    let mut last_and = 0;
    while j < len {
        s[i] = s[j];
        if s[i] == b';' {
            let ch = match &s[last_and..=i] {
                b"&quot;" => b'"',
                b"&apos;" => b'\'',
                b"&amp;" => b'&',
                b"&gt;" => b'>',
                b"&lt;" => b'<',
                b"&frasl;" => b'/',
                _ => 0,
            };
            if ch != 0 {
                i = last_and;
                s[i] = ch;
                last_and += 1;
            }
        } else if s[i] == b'&' {
            last_and = i;
        }
        i += 1;
        j += 1;
    }
    unsafe { s.set_len(i); }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    fn test(func: fn(text: String) -> String) {
        assert_eq!(func(String::from("&amp;gt;")), String::from("&gt;"));
        assert_eq!(func(String::from("&&gt;")), String::from("&>"));
        assert_eq!(func(String::from(" &frasl; &quot; &apos; ZooP)x:6~")), String::from(" / \" ' ZooP)x:6~"));
        assert_eq!(func(String::from("&abc")), String::from("&abc"));
        assert_eq!(func(String::from("&amp; is an HTML entity but &ambassador; is not.")), String::from("& is an HTML entity but &ambassador; is not."));
        assert_eq!(func(String::from("and I quote: &quot;...&quot;")), String::from("and I quote: \"...\""));
    }
    test(entity_parser);
    test(entity_parser2);
}
