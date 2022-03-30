//! 字母移位

pub fn shifting_letters(s: String, mut shifts: Vec<i32>) -> String {
    let mut s = s.into_bytes();
    let len = shifts.len();
    shifts[len - 1] %= 26;
    for i in (0..len - 1).rev() {
        shifts[i] = (shifts[i] + shifts[i + 1]) % 26;
    }
    for i in 0..len {
        s[i] = (s[i] - b'a' + shifts[i] as u8) % 26 + b'a';
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(shifting_letters(String::from("abc"), vec![3, 5, 9]), String::from("rpl"));
    assert_eq!(shifting_letters(String::from("aaa"), vec![1, 2, 3]), String::from("gfd"));
}
