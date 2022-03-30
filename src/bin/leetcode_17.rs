//! 电话号码的字母组合

use leetcode::svec;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 { return vec![]; }
    let map: Vec<Vec<u8>> = vec![vec![b'a', b'b', b'c'],
                                 vec![b'd', b'e', b'f'],
                                 vec![b'g', b'h', b'i'],
                                 vec![b'j', b'k', b'l'],
                                 vec![b'm', b'n', b'o'],
                                 vec![b'p', b'q', b'r', b's'],
                                 vec![b't', b'u', b'v'],
                                 vec![b'w', b'x', b'y', b'z']];
    let chars = digits.as_bytes();
    let mut ans = vec![vec![]];
    for n in chars {
        let mut tmp = vec![];
        for i in ans {
            for &char in &map[(n - b'2') as usize] {
                let mut t = i.clone();
                t.push(char);
                tmp.push(t);
            }
        }
        ans = tmp;
    }
    ans.into_iter().map(|x| unsafe { String::from_utf8_unchecked(x) }).collect()
}


fn main() {
    assert_eq!(letter_combinations(String::from("23")), svec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    assert_eq!(letter_combinations(String::from("")), svec![]);
    assert_eq!(letter_combinations(String::from("2")), svec!["a", "b", "c"]);
}
