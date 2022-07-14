//! 锯齿迭代器

struct ZigzagIterator {
    vec: Vec<Vec<i32>>,
    idx: Vec<usize>,
    cur: usize,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let cur = if v1.is_empty() { 1 } else { 0 };
        Self { vec: vec![v1, v2], idx: vec![0, 0], cur: cur }
    }

    fn next(&mut self) -> i32 {
        let result = self.vec[self.cur][self.idx[self.cur]];
        self.idx[self.cur] += 1;
        self.cur = (self.cur + 1) % self.vec.len();
        if self.has_next() {
            while self.idx[self.cur] == self.vec[self.cur].len() {
                self.cur = (self.cur + 1) % self.vec.len();
            }
        }
        result
    }

    fn has_next(&self) -> bool {
        let mut i = self.cur;
        while self.idx[i] == self.vec[i].len() {
            i = (i + 1) % self.idx.len();
            if i == self.cur { return false; }
        }
        true
    }
}


fn main() {
    let mut zi = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
    let mut output = vec![];
    while zi.has_next() {
        output.push(zi.next());
    }
    assert_eq!(output, [1, 3, 2, 4, 5, 6]);
}
