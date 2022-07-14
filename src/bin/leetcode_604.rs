//! 迭代压缩字符串

struct StringIterator {
    list: Vec<(u8, i32)>,
}


impl StringIterator {
    fn new(s: String) -> Self {
        let s = s.as_bytes();
        let mut i = 0;
        let mut list = vec![];
        let len = s.len();
        while i < len {
            let ch = s[i];
            let mut num = 0;
            i += 1;
            while i < len && s[i].is_ascii_digit() {
                num = num * 10 + (s[i] - b'0') as i32;
                i += 1;
            }
            list.push((ch, num));
        }
        list.reverse();
        Self { list }
    }

    fn next(&mut self) -> char {
        if self.list.is_empty() { return ' '; }
        let (ch, num) = self.list.last_mut().unwrap();
        let ch = *ch;
        *num -= 1;
        if *num == 0 {
            self.list.pop();
        }
        ch as char
    }

    fn has_next(&self) -> bool {
        !self.list.is_empty()
    }
}


fn main() {
    let mut si = StringIterator::new(String::from("L1e2t1C1o1d1e1"));
    assert_eq!(si.next(), 'L');
    assert_eq!(si.next(), 'e');
    assert_eq!(si.next(), 'e');
    assert_eq!(si.next(), 't');
    assert_eq!(si.next(), 'C');
    assert_eq!(si.next(), 'o');
    assert_eq!(si.has_next(), true);
    assert_eq!(si.next(), 'd');
    assert_eq!(si.has_next(), true);
}
