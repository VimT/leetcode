//! 键盘行

use leetcode::svec;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    fn char_int(c: u8) -> u8 {
        match c {
            b'q' | b'w' | b'e' | b'r' | b't' | b'y' | b'u' | b'i' | b'o' | b'p' => 1,
            b'Q' | b'W' | b'E' | b'R' | b'T' | b'Y' | b'U' | b'I' | b'O' | b'P' => 1,
            b'a' | b's' | b'd' | b'f' | b'g' | b'h' | b'j' | b'k' | b'l' => 2,
            b'A' | b'S' | b'D' | b'F' | b'G' | b'H' | b'J' | b'K' | b'L' => 2,
            b'z' | b'x' | b'c' | b'v' | b'b' | b'n' | b'm' => 3,
            b'Z' | b'X' | b'C' | b'V' | b'B' | b'N' | b'M' => 3,
            _ => panic!()
        }
    }
    words.into_iter().filter(|x| {
        let s = x.as_bytes();
        let n = char_int(s[0]);
        for &i in &s[1..] {
            if char_int(i) != n {
                return false;
            }
        }
        true
    }
    ).collect()
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["Hello","Alaska","Dad","Peace"]), vec!["Alaska", "Dad"]);
        assert_eq!(func(svec!["omk"]), Vec::<String>::new());
        assert_eq!(func(svec!["adsdf","sfd"]), vec!["adsdf", "sfd"]);
    }
    test(find_words);
}
