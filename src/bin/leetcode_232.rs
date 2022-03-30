//! 用栈实现队列


use std::collections::VecDeque;

struct MyStack {
    s1: VecDeque<i32>,
    s2: VecDeque<i32>,
}


impl MyStack {
    fn new() -> Self {
        MyStack {
            s1: VecDeque::new(),
            s2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        while !self.s1.is_empty() {
            self.s2.push_back(self.s1.pop_back().unwrap());
        }
        self.s1.push_back(x);
        while !self.s2.is_empty() {
            self.s1.push_back(self.s2.pop_back().unwrap());
        }
    }
    fn pop(&mut self) -> i32 {
        self.s1.pop_back().unwrap()
    }

    fn peek(&mut self) -> i32 {
        *self.s1.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}

fn main() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.peek(), 1);
    assert_eq!(s.pop(), 1);
    assert_eq!(s.empty(), false);
}
