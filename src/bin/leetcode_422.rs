//! 有效的单词方块

use leetcode::svec;

pub fn valid_word_square(words: Vec<String>) -> bool {
    let m = words.len();
    for i in 0..m {
        for j in 0..words[i].len() {
            if j >= m || i >= words[j].len() || words[i].as_bytes()[j] != words[j].as_bytes()[i] {
                return false;
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(words: Vec<String>) -> bool) {
        assert_eq!(func(svec!["abcd","bnrt","crmy","dtye"]), true);
        assert_eq!(func(svec!["abcd","bnrt","crm","dt"]), true);
        assert_eq!(func(svec!["ball","area","read","lady"]), false);
    }
    test(valid_word_square);
}
