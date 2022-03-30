//! 设计循环队列

struct MyCircularQueue {
    v: Vec<i32>,
    len: usize,
    left: usize,
    right: usize,
}


impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self { v: vec![0; k as usize], len: 0, left: 0, right: 0 }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() { return false; }
        self.v[self.right] = value;
        self.right = (self.right + 1) % self.v.len();
        self.len += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() { return false; }
        self.left = (self.left + 1) % self.v.len();
        self.len -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() { return -1; }
        self.v[self.left]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() { return -1; }
        self.v[(self.right + self.v.len() - 1) % self.v.len()]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.v.len()
    }
}


fn main() {
    let mut cq = MyCircularQueue::new(3); // 设置长度为 3
    assert_eq!(cq.en_queue(1), true);  // 返回 true
    assert_eq!(cq.en_queue(2), true);  // 返回 true
    assert_eq!(cq.en_queue(3), true);  // 返回 true
    assert_eq!(cq.en_queue(4), false);  // 返回 false，队列已满
    assert_eq!(cq.rear(), 3);  // 返回 3
    assert_eq!(cq.is_full(), true);  // 返回 true
    assert_eq!(cq.de_queue(), true);  // 返回 true
    assert_eq!(cq.en_queue(4), true);  // 返回 true
    assert_eq!(cq.rear(), 4);  // 返回 4

    cq = MyCircularQueue::new(7);
    assert_eq!(cq.en_queue(0), true);
    assert_eq!(cq.front(), 0);
    assert_eq!(cq.en_queue(4), true);
    assert_eq!(cq.rear(), 4);
    assert_eq!(cq.en_queue(6), true);
    assert_eq!(cq.en_queue(3), true);
    assert_eq!(cq.rear(), 3);
    assert_eq!(cq.de_queue(), true);
    assert_eq!(cq.front(), 4);
    assert_eq!(cq.de_queue(), true);
    assert_eq!(cq.front(), 6);
}
