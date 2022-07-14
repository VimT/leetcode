//! 最大栈

struct MaxStack {
    s: Vec<(i32, i32)>,
}

impl MaxStack {
    fn new() -> Self {
        Self { s: vec![] }
    }

    fn push(&mut self, x: i32) {
        let max = self.s.last().map(|last| last.1.max(x)).unwrap_or(x);
        self.s.push((x, max));
    }

    fn pop(&mut self) -> i32 {
        self.s.pop().unwrap().0
    }

    fn top(&self) -> i32 {
        self.s.last().unwrap().0
    }

    fn peek_max(&self) -> i32 {
        self.s.last().unwrap().1
    }

    fn pop_max(&mut self) -> i32 {
        let max = self.peek_max();
        let mut b = vec![];
        loop {
            let num = self.pop();
            if num == max {
                break;
            } else {
                b.push(num);
            }
        }
        for i in b.into_iter().rev() {
            self.push(i);
        }
        max
    }
}


fn main() {
    let mut stk = MaxStack::new();
    stk.push(5);
    stk.push(1);
    assert_eq!(stk.top(), 1);
    assert_eq!(stk.pop_max(), 5);
    assert_eq!(stk.peek_max(), 1);
    assert_eq!(stk.pop(), 1);
}
