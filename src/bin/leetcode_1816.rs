//! 截断句子

pub fn truncate_sentence(s: String, k: i32) -> String {
    let mut cnt = 0;
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b' ' {
            cnt += 1;
            if cnt == k {
                return s[..i].to_string();
            }
        }
    }
    s
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("Hello how are you Contestant"), 4), String::from("Hello how are you"));
        assert_eq!(func(String::from("What is the solution to this problem"), 4), String::from("What is the solution"));
        assert_eq!(func(String::from("chopper is not a tanuki"), 5), String::from("chopper is not a tanuki"));
    }
    test(truncate_sentence);
}
