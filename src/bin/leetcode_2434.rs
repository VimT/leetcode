//! 使用机器人打印字典序最小的字符串

pub fn robot_with_string(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = Vec::with_capacity(s.len());
    let mut i = 0;
    let mut stack = vec![];
    for ch in b'a'..=b'z' {
        while !stack.is_empty() && *stack.last().unwrap() <= ch {
            result.push(stack.pop().unwrap());
        }
        while i < len {
            // 向右
            let mut j = i;
            while j < len && s[j] != ch {
                j += 1;
            }
            if j == len { break; }
            for k in i..j {
                stack.push(s[k]);
            }
            i = j + 1;
            result.push(ch);
        }
    }
    while let Some(ch) = stack.pop() {
        result.push(ch);
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(robot_with_string(String::from("vzhofnpo")), String::from("fnohopzv"));
    assert_eq!(robot_with_string(String::from("zza")), String::from("azz"));
    assert_eq!(robot_with_string(String::from("bac")), String::from("abc"));
    assert_eq!(robot_with_string(String::from("bdda")), String::from("addb"));
}
