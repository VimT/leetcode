//! 压缩字符串

pub fn compress(chars: &mut Vec<char>) -> i32 {
    if chars.is_empty() { return 0; }
    let mut wi = 0;
    let mut cur_char = chars[0];
    let mut cnt = 1;
    let len = chars.len();
    for i in 1..=len {
        if i < len && chars[i] == cur_char {
            cnt += 1;
        } else {
            chars[wi] = cur_char;
            wi += 1;
            if cnt > 1 {
                for ch in cnt.to_string().chars() {
                    chars[wi] = ch;
                    wi += 1;
                }
            }
            if i == len { break; }
            cur_char = chars[i];
            cnt = 1;
        }
    }
    wi as i32
}

fn main() {
    fn cal(before: String, target: String, len: usize) {
        let mut before = before.chars().collect();
        let target: Vec<char> = target.chars().collect();
        assert_eq!(compress(&mut before), len as i32);
        assert_eq!(before[..len], target);
    }
    cal(String::from("aabbccc"), String::from("a2b2c3"), 6);
    cal(String::from("a"), String::from("a"), 1);
    cal(String::from("abbbbbbbbbbbb"), String::from("ab12"), 4);
    cal(String::from("aaabbaa"), String::from("a3b2a2"), 6);
}
