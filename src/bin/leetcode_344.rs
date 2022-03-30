//! 反转字符串

pub fn reverse_string(s: &mut Vec<char>) {
    if s.len() <= 1 { return; }
    let mut p1 = 0;
    let mut p2 = s.len() - 1;
    while p1 < p2 {
        s.swap(p1, p2);
        p1 += 1;
        p2 -= 1;
    }
}


fn main() {
    fn test(func: fn(s: &mut Vec<char>)) {
        let help = |mut s: Vec<char>| {
            func(&mut s);
            s
        };
        assert_eq!(help(vec!['h', 'e', 'l', 'l', 'o']), ['o', 'l', 'l', 'e', 'h']);
        assert_eq!(help(vec!['H', 'a', 'n', 'n', 'a', 'h']), ['h', 'a', 'n', 'n', 'a', 'H']);
    }
    test(reverse_string);
}
