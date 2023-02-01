//! 设计一个支持增量操作的栈

struct CustomStack {
    s: Vec<i32>,
    size: usize,
}

impl CustomStack {
    fn new(size: i32) -> Self {
        Self { s: Vec::with_capacity(size as usize), size: size as usize }
    }

    fn push(&mut self, x: i32) {
        if self.s.len() < self.size {
            self.s.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.s.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let len = self.s.len();
        for x in &mut self.s[..(k as usize).min(len)] {
            *x += val;
        }
    }
}

fn main() {
    let mut stk = CustomStack::new(3); // 栈是空的 []
    stk.push(1);                          // 栈变为 [1]
    stk.push(2);                          // 栈变为 [1, 2]
    assert_eq!(stk.pop(), 2);                            // 返回 2 --> 返回栈顶值 2，栈变为 [1]
    stk.push(2);                          // 栈变为 [1, 2]
    stk.push(3);                          // 栈变为 [1, 2, 3]
    stk.push(4);                          // 栈仍然是 [1, 2, 3]，不能添加其他元素使栈大小变为 4
    stk.increment(5, 100);                // 栈变为 [101, 102, 103]
    stk.increment(2, 100);                // 栈变为 [201, 202, 103]
    assert_eq!(stk.pop(), 103);                            // 返回 103 --> 返回栈顶值 103，栈变为 [201, 202]
    assert_eq!(stk.pop(), 202);                            // 返回 202 --> 返回栈顶值 202，栈变为 [201]
    assert_eq!(stk.pop(), 201);                            // 返回 201 --> 返回栈顶值 201，栈变为 []
    assert_eq!(stk.pop(), -1);                            // 返回 -1 --> 栈为空，返回 -1
}
