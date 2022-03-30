//! 考场就座


use std::collections::BTreeSet;

struct ExamRoom {
    v: BTreeSet<i32>,
    len: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self { v: BTreeSet::new(), len: n }
    }

    fn seat(&mut self) -> i32 {
        if self.v.is_empty() {
            self.v.insert(0);
            return 0;
        }
        let mut prev = *self.v.iter().next().unwrap();
        let mut idx = 0;
        let mut max = prev;
        for &k in &self.v {
            if (k - prev) / 2 > max {
                idx = (k + prev) / 2;
                max = (k - prev) / 2;
            }
            prev = k;
        }
        if self.len - 1 - prev > max {
            idx = self.len - 1;
        }
        self.v.insert(idx);
        idx
    }

    fn leave(&mut self, p: i32) {
        self.v.remove(&p);
    }
}

fn main() {
    let mut ex = ExamRoom::new(10);
    assert_eq!(ex.seat(), 0); //没有人在考场里，那么学生坐在 0 号座位上。
    assert_eq!(ex.seat(), 9); //学生最后坐在 9 号座位上。
    assert_eq!(ex.seat(), 4); //学生最后坐在 4 号座位上。
    assert_eq!(ex.seat(), 2); //学生最后坐在 2 号座位上。
    ex.leave(4);
    assert_eq!(ex.seat(), 5); //学生最后坐在 5 号座位上。
}