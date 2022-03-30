//! 向字符串添加空格

pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = vec![b'0'; len + spaces.len()];
    let mut si = spaces.len() as i32 - 1;
    let mut ri = result.len() as i32 - 1;
    for i in (0..len).rev() {
        result[ri as usize] = s[i];
        ri -= 1;
        if si >= 0 && i == spaces[si as usize] as usize {
            result[ri as usize] = b' ';
            si -= 1;
            ri -= 1;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]), "Leetcode Helps Me Learn");
    assert_eq!(add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9]), "i code in py thon");
    assert_eq!(add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6]), " s p a c i n g");
}