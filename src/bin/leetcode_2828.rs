//! 判别首字母缩略词

pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    words.into_iter().map(|x| x.as_bytes()[0]).collect::<Vec<u8>>() == s.as_bytes()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, s: String) -> bool) {
        assert_eq!(func(svec!["alice","bob","charlie"], String::from("abc")), true);
        assert_eq!(func(svec!["an","apple"], String::from("a")), false);
        assert_eq!(func(svec!["never","gonna","give","up","on","you"], String::from("ngguoy")), true);
    }
    test(is_acronym);
}
