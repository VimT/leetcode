//! 最小栈

#[derive(Default)]
struct MinStack {
    stack: Vec<i32>,
    small_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.small_stack.is_empty() {
            self.small_stack.push(x)
        } else {
            let &last_small = self.small_stack.last().unwrap();
            if last_small > x {
                self.small_stack.push(x);
            } else { self.small_stack.push(last_small) }
        }
    }

    fn pop(&mut self) {
        self.small_stack.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.small_stack.last().unwrap();
    }
}

fn main() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.get_min(), -3);
    s.pop();
    assert_eq!(s.top(), 0);
    assert_eq!(s.get_min(), -2);
}
