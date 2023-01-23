//! 设计循环双端队列

struct MyCircularDeque {
    tail: usize,
    head: usize,
    buf: Vec<i32>,
    len: usize,
}


impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self { tail: 0, head: 0, buf: vec![0; k as usize], len: 0 }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.len == self.buf.len() {
            return false;
        }
        self.head = (self.head + self.buf.len() - 1) % self.buf.len();
        self.buf[self.head] = value;
        self.len += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.len == self.buf.len() {
            return false;
        }
        self.buf[self.tail] = value;
        self.tail = (self.tail + self.buf.len() + 1) % self.buf.len();
        self.len += 1;
        true
    }

    #[allow(dead_code)]
    fn delete_front(&mut self) -> bool {
        if self.is_empty() { return false; }
        self.head = (self.head + self.buf.len() + 1) % self.buf.len();
        self.len -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() { return false; }
        self.tail = (self.tail + self.buf.len() - 1) % self.buf.len();
        self.len -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() { return -1; }
        self.buf[self.head]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() { return -1; }
        self.buf[(self.tail + self.buf.len() - 1) % self.buf.len()]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.buf.len()
    }
}

fn main() {
    let mut obj = MyCircularDeque::new(3);
    assert_eq!(obj.insert_last(1), true);
    assert_eq!(obj.insert_last(2), true);
    assert_eq!(obj.insert_front(3), true);
    assert_eq!(obj.insert_front(4), false);
    assert_eq!(obj.get_rear(), 2);
    assert_eq!(obj.is_full(), true);
    assert_eq!(obj.delete_last(), true);
    assert_eq!(obj.insert_front(4), true);
    assert_eq!(obj.get_front(), 4);

    let mut obj = MyCircularDeque::new(4);
    assert_eq!(obj.insert_front(4), true);
    assert_eq!(obj.delete_last(), true);
    assert_eq!(obj.get_rear(), -1);
}
