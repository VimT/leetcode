//! 删去字符串中的元音

pub fn remove_vowels(s: String) -> String {
    s.chars().filter(|x| !matches!(x, 'a'|'e'|'i'|'o'|'u')).collect()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("leetcodeisacommunityforcoders")), String::from("ltcdscmmntyfrcdrs"));
        assert_eq!(func(String::from("aeiou")), String::from(""));
    }
    test(remove_vowels);
}
