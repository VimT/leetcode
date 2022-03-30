struct RLEIterator {
    encoding: Vec<i32>,
    cur: usize,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        RLEIterator { encoding, cur: 0 }
    }

    fn next(&mut self, mut n: i32) -> i32 {
        while n > 0 && self.cur < self.encoding.len() {
            if self.encoding[self.cur] >= n {
                self.encoding[self.cur] -= n;
                return self.encoding[self.cur + 1];
            } else {
                n -= self.encoding[self.cur];
                self.encoding[self.cur] = 0;
                self.cur += 2;
            }
        }
        -1
    }
}

fn main() {
    let mut rle = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    assert_eq!(rle.next(2), 8);
    assert_eq!(rle.next(1), 8);
    assert_eq!(rle.next(1), 5);
    assert_eq!(rle.next(-1), -1);
}
