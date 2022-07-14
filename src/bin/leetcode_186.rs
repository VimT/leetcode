//! 翻转字符串里的单词 II

pub fn reverse_words(s: &mut Vec<char>) {
    fn reverse(s: &mut [char]) {
        let mut start = 0;
        let mut end = s.len();
        while start + 1 < end {
            s.swap(start, end - 1);
            start += 1;
            end -= 1;
        }
    }
    let mut start = 0;
    let len = s.len();
    for i in 0..len {
        if s[i] == ' ' {
            reverse(&mut s[start..i]);
            start = i + 1;
        }
    }
    reverse(&mut s[start..]);
    reverse(s);
}

fn main() {
    fn test(func: fn(s: &mut Vec<char>)) {
        let help = |mut s: Vec<char>| {
            func(&mut s);
            s
        };
        assert_eq!(help(vec!['t', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e']), ['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e']);
        assert_eq!(help(vec!['a']), ['a']);
    }
    test(reverse_words);
}
