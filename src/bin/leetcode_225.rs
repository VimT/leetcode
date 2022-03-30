//! 用队列实现栈


use std::collections::VecDeque;

struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}


impl MyStack {
    fn new() -> Self {
        MyStack {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue1.push_back(x);
        while !self.queue2.is_empty() {
            self.queue1.push_back(self.queue2.pop_front().unwrap());
        }
        std::mem::swap(&mut self.queue1, &mut self.queue2);
    }

    fn pop(&mut self) -> i32 {
        self.queue2.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        *self.queue2.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue2.is_empty()
    }
}

fn main() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.top(), 2);
    assert_eq!(s.pop(), 2);
    assert_eq!(s.empty(), false);
}
