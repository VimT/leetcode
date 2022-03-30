//! 验证回文串

pub fn is_palindrome(s: String) -> bool {
    let mut vec = Vec::with_capacity(s.len());
    let s = s.to_lowercase();
    let mut len = 0;
    for i in s.chars() {
        if i.is_ascii_alphanumeric() {
            vec.push(i);
            len += 1;
        }
    }
    if len == 0 { return true; }
    if len == 1 { return true; }
    let mut p1 = 0;
    let mut p2 = len - 1;
    while p1 < p2 {
        if vec[p1 as usize] != vec[p2 as usize] { return false; }
        p1 += 1;
        p2 -= 1;
    }
    return true;
}

pub fn is_palindrome_unsafe(s: String) -> bool {
    let s = s.to_lowercase();
    let vec = s.as_bytes()
        .iter()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| *x)
        .collect::<Vec<_>>();
    let len = vec.len();
    if len == 0 { return true; }

    let mut p1 = 0;
    let mut p2 = len - 1;
    while p1 < p2 {
        if vec[p1 as usize] != vec[p2 as usize] { return false; }
        p1 += 1;
        p2 -= 1;
    }
    return true;
}


fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("A man, a plan, a canal: Panama")), true);
        assert_eq!(func(String::from("race a car")), false);
        assert_eq!(func(String::from(" ")), true);
    }
    test(is_palindrome);
    test(is_palindrome_unsafe);
}
