//! 找出第 K 个字符 I

pub fn kth_character(k: i32) -> char {
    let mut s = vec![];
    s.push(b'a');
    while s.len() < k as usize {
        for j in 0..s.len() {
            s.push(if s[j] < b'z' { s[j] + 1 } else { b'a' });
        }
    }
    s[k as usize - 1] as char
}

fn main() {
    fn test(func: fn(k: i32) -> char) {
        assert_eq!(func(5), 'b');
        assert_eq!(func(10), 'c');
    }
    test(kth_character);
}
