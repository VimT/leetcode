//! 统计星号

pub fn count_asterisks(s: String) -> i32 {
    let mut is_in = false;
    let mut result = 0;
    for &ch in s.as_bytes() {
        match ch {
            b'|' => { is_in = !is_in }
            b'*' if !is_in => { result += 1 }
            _ => ()
        }
    }
    result
}

fn main() {
    assert_eq!(count_asterisks(String::from("l|*e*et|c**o|*de|")), 2);
    assert_eq!(count_asterisks(String::from("iamprogrammer")), 0);
    assert_eq!(count_asterisks(String::from("yo|uar|e**|b|e***au|tifu|l")), 5);
}
