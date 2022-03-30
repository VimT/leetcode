use std::collections::VecDeque;

struct CQueue {
    pub s1: VecDeque<i32>,
    pub s2: VecDeque<i32>,
}

impl CQueue {
    fn new() -> Self {
        CQueue { s1: VecDeque::new(), s2: VecDeque::new() }
    }

    fn append_tail(&mut self, value: i32) {
        self.s1.push_back(value)
    }

    fn delete_head(&mut self) -> i32 {
        if self.s2.is_empty() {
            if self.s1.is_empty() {
                return -1;
            } else {
                while !self.s1.is_empty() {
                    self.s2.push_back(self.s1.pop_back().unwrap());
                }
            }
        }
        self.s2.pop_back().unwrap()
    }
}

fn main() {
    let mut obj = CQueue::new();
    obj.append_tail(1);
    obj.append_tail(2);
    obj.append_tail(3);
    let ret_2: i32 = obj.delete_head();
    assert_eq!(ret_2, 1);
    obj.append_tail(4);
    assert_eq!(obj.delete_head(), 2);
    obj.append_tail(5);
    assert_eq!(obj.delete_head(), 3);
    obj.delete_head();
    obj.delete_head();
    obj.delete_head();
    obj.delete_head();
    assert_eq!(obj.delete_head(), -1);
}
