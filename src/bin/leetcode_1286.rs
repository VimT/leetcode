//! 字母组合迭代器

struct CombinationIterator {
    pos: Vec<usize>,
    finish: bool,
    chars: Vec<u8>,
}


/// 全排列迭代器
impl CombinationIterator {
    fn new(characters: String, len: i32) -> Self {
        Self {
            pos: (0..len as usize).collect(),
            finish: false,
            chars: characters.into_bytes(),
        }
    }

    fn next(&mut self) -> String {
        let result: Vec<u8> = self.pos.iter().map(|&x| self.chars[x]).collect();
        let mut i = self.pos.len();
        for k in (0..self.pos.len()).rev() {
            if self.pos[k] != self.chars.len() - self.pos.len() + k {
                i = k;
                break;
            }
        }
        if i == self.pos.len() { self.finish = true; } else {
            self.pos[i] += 1;
            for j in i + 1..self.pos.len() {
                self.pos[j] = self.pos[j - 1] + 1;
            }
        }
        unsafe { String::from_utf8_unchecked(result) }
    }

    fn has_next(&self) -> bool {
        !self.finish
    }
}


fn main() {
    let mut iterator = CombinationIterator::new(String::from("abc"), 2);
    assert_eq!(iterator.next(), "ab"); // 返回 "ab"
    assert_eq!(iterator.has_next(), true); // 返回 true
    assert_eq!(iterator.next(), "ac"); // 返回 "ac"
    assert_eq!(iterator.has_next(), true); // 返回 true
    assert_eq!(iterator.next(), "bc"); // 返回 "bc"
    assert_eq!(iterator.has_next(), false); // 返回 false
}
