//! 使括号有效的最少添加

pub fn min_add_to_make_valid(s: String) -> i32 {
    let s = s.as_bytes();
    let mut st = vec![];
    let mut result = 0;
    for i in 0..s.len() {
        if s[i] == b'(' {
            st.push(i);
        } else {
            if st.is_empty() {
                result += 1;
            } else {
                st.pop().unwrap();
            }
        }
    }
    result += st.len() as i32;
    result
}

fn main() {
    assert_eq!(min_add_to_make_valid(String::from("())")), 1);
    assert_eq!(min_add_to_make_valid(String::from("(((")), 3);
    assert_eq!(min_add_to_make_valid(String::from("()))((")), 4);
    assert_eq!(min_add_to_make_valid(String::from("()")), 0);
}
