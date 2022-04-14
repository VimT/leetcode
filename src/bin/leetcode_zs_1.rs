//! 招商银行-01. 文本编辑程序设计

pub fn delete_text(article: String, index: i32) -> String {
    let mut s = article.into_bytes();
    if s[index as usize] != b' ' {
        let mut left = index as usize;
        while left > 0 && s[left - 1] != b' ' {
            left -= 1;
        }
        let mut right = index as usize;
        while right < s.len() && s[right] != b' ' {
            right += 1;
        }
        if right < s.len() { right += 1; }
        if right == s.len() {
            if left > 0 { left -= 1; }
        }
        s.drain(left..right);
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(delete_text(String::from("Hello"), 0), "");
    assert_eq!(delete_text(String::from("e RSg c R cf"), 10), "e RSg c R");
    assert_eq!(delete_text(String::from("Singing dancing in the rain"), 10), "Singing in the rain");
    assert_eq!(delete_text(String::from("Hello World"), 2), "World");
    assert_eq!(delete_text(String::from("Hello World"), 5), "Hello World");
}
