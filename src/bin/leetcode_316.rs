//! 去除重复字母


pub fn remove_duplicate_letters(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut letter_num = vec![0; 26];
    for i in 0..len {
        letter_num[(s[i] - b'a') as usize] += 1;
    }
    let mut stack = Vec::with_capacity(len);
    let mut visited = vec![false; 26];
    for i in 0..len {
        let index = (s[i] - b'a') as usize;
        if !visited[index] {
            while !stack.is_empty() && *stack.last().unwrap() > s[i] {
                let pop = *stack.last().unwrap();
                if letter_num[(pop - b'a') as usize] > 0 {
                    visited[(pop - b'a') as usize] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(s[i]);
            visited[index] = true;
        }
        letter_num[index] -= 1;
    }
    unsafe { String::from_utf8_unchecked(stack) }
}


fn main() {
    assert_eq!(remove_duplicate_letters(String::from("bcabc")), String::from("abc"));
    assert_eq!(remove_duplicate_letters(String::from("cbacdcbc")), String::from("acdb"));
}
