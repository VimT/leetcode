//! 用 Read4 读取 N 个字符

use std::cell::RefCell;

struct Solution {
    content: Vec<char>,
    idx: RefCell<usize>,
}

impl Solution {
    fn new(content: &str) -> Self {
        Self {
            content: content.chars().collect(),
            idx: RefCell::new(0),
        }
    }
    fn read4(&self, buf4: &mut [char]) -> i32 {
        let mut cnt = 0;
        for i in 0..4 {
            if *self.idx.borrow() == self.content.len() { break; }
            buf4[i] = self.content[*self.idx.borrow()];
            *self.idx.borrow_mut() += 1;
            cnt += 1;
        }
        cnt
    }
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut idx = 0;
        let mut tmp = vec!['0'; 4];
        let mut tidx = 0;
        let mut tsize = 0;
        while idx < n {
            while tidx < tsize && idx < n {
                buf[idx as usize] = tmp[tidx as usize];
                idx += 1;
                tidx += 1;
            }
            if idx < n {
                tsize = self.read4(&mut tmp);
                tidx = 0;
                if tsize == 0 { break; }
            }
        }
        idx
    }
}

fn main() {
    fn test(content: &str, read_len: usize, real_len: usize) {
        let case = Solution::new(content);
        let mut buf = vec!['0'; read_len];
        let size = case.read(&mut buf, read_len as i32);
        assert_eq!(size as usize, real_len);
        assert_eq!(buf[..real_len], case.content[..real_len]);
    }
    test("abc", 4, 3);
    test("abcde", 5, 5);
    test("abcdABCD1234", 12, 12);
}
