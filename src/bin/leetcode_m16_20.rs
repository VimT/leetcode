//! 面试题 16.20. T9键盘

use leetcode::svec;

pub fn get_valid_t9_words(num: String, words: Vec<String>) -> Vec<String> {
    fn num2char(word: &String) -> Vec<u8> {
        word.as_bytes().iter().map(|&x|
            match x {
                b'a' => 2,
                b'b' => 2,
                b'c' => 2,
                b'd' => 3,
                b'e' => 3,
                b'f' => 3,
                b'g' => 4,
                b'h' => 4,
                b'i' => 4,
                b'j' => 5,
                b'k' => 5,
                b'l' => 5,
                b'm' => 6,
                b'n' => 6,
                b'o' => 6,
                b'p' => 7,
                b'q' => 7,
                b'r' => 7,
                b's' => 7,
                b't' => 8,
                b'u' => 8,
                b'v' => 8,
                b'w' => 9,
                b'x' => 9,
                b'y' => 9,
                b'z' => 9,
                _ => { panic!() }
            }
        ).collect()
    }
    let num: Vec<u8> = num.as_bytes().iter().map(|&x| x - b'0').collect();
    words.into_iter().filter(|x| {
        num2char(x) == num
    }).collect()
}

fn main() {
    assert_eq!(get_valid_t9_words("8733".to_string(), svec!["tree", "used"]), svec!["tree", "used"]);
    assert_eq!(get_valid_t9_words("2".to_string(), svec!["a", "b", "c", "d"]), svec!["a", "b", "c"]);
}