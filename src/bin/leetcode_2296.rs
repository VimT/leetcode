//! 设计一个文本编辑器

struct TextEditor {
    text: Vec<u8>,
    i: usize,
}


impl TextEditor {
    fn new() -> Self {
        Self { text: Vec::new(), i: 0 }
    }

    fn add_text(&mut self, text: String) {
        let mut new_text = Vec::with_capacity(self.text.len() + text.len());
        new_text.extend_from_slice(&self.text[..self.i]);
        new_text.extend_from_slice(text.as_bytes());
        new_text.extend_from_slice(&self.text[self.i..]);
        self.text = new_text;
        self.i += text.len();
    }

    fn delete_text(&mut self, mut k: i32) -> i32 {
        let mut new_text = Vec::with_capacity(self.text.len());
        if (self.i as i32) < k {
            k = self.i as i32;
        }
        new_text.extend_from_slice(&self.text[..self.i - k as usize]);
        new_text.extend_from_slice(&self.text[self.i..]);
        self.text = new_text;
        self.i -= k as usize;
        k
    }

    fn cursor_left(&mut self, k: i32) -> String {
        self.i = (self.i as i32 - k).max(0) as usize;
        unsafe { String::from_utf8_unchecked(self.text[(self.i as i32 - 10).max(0) as usize..self.i].to_vec()) }
    }

    fn cursor_right(&mut self, k: i32) -> String {
        self.i = (self.i as i32 + k).min(self.text.len() as i32) as usize;
        unsafe { String::from_utf8_unchecked(self.text[(self.i as i32 - 10).max(0) as usize..self.i].to_vec()) }
    }
}


use std::ptr::null_mut;

struct Node {
    prev: *mut Node,
    next: *mut Node,
    val: u8,
}

impl Node {
    fn new(val: u8) -> *mut Node {
        let node = Box::new(Node { prev: null_mut(), next: null_mut(), val });
        Box::into_raw(node)
    }

    unsafe fn remove(node: *mut Node) {
        let next = (*node).next;
        let prev = (*node).prev;
        (*prev).next = next;
        (*next).prev = prev;
    }

    unsafe fn insert(node: *mut Node, new: *mut Node) {
        let ori_next = (*node).next;
        (*new).next = ori_next;
        (*new).prev = node;
        (*ori_next).prev = new;
        (*node).next = new;
    }
}

/// 链表
struct TextEditor2 {
    head: *mut Node,
    cur: *mut Node,
}


impl TextEditor2 {
    fn new() -> Self {
        unsafe {
            let head = Node::new(0);
            (*head).prev = head;
            (*head).next = head;
            Self { head, cur: head }
        }
    }

    fn add_text(&mut self, text: String) {
        unsafe {
            for &ch in text.as_bytes() {
                Node::insert(self.cur, Node::new(ch));
                self.cur = (*self.cur).next;
            }
        }
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        unsafe {
            if self.cur == self.head { return 0; }
            for i in 0..k {
                let prev = (*self.cur).prev;
                Node::remove(self.cur);
                self.cur = prev;
                if self.cur == self.head {
                    return i + 1;
                }
            }
            k
        }
    }

    fn cursor_left(&mut self, k: i32) -> String {
        unsafe {
            for _ in 0..k {
                if self.cur == self.head { break; }
                self.cur = (*self.cur).prev;
            }
            let mut result = vec![];
            let mut p = self.cur;
            for _ in 0..10 {
                if p == self.head { break; }
                result.push((*p).val);
                p = (*p).prev;
            }
            result.reverse();
            String::from_utf8_unchecked(result)
        }
    }

    fn cursor_right(&mut self, k: i32) -> String {
        unsafe {
            for _ in 0..k {
                if (*self.cur).next == self.head { break; }
                self.cur = (*self.cur).next;
            }
            let mut result = vec![];
            let mut p = self.cur;
            for _ in 0..10 {
                if p == self.head { break; }
                result.push((*p).val);
                p = (*p).prev;
            }
            result.reverse();
            String::from_utf8_unchecked(result)
        }
    }

    fn print(&self) {
        unsafe {
            let mut p = (*self.head).next;
            let mut result = vec![];
            if self.cur == self.head { result.push(b'|'); }
            while p != self.head {
                result.push((*p).val);
                if p == self.cur {
                    result.push(b'|');
                }
                p = (*p).next;
            }
            println!("{}", String::from_utf8_unchecked(result));
        }
    }
}

/// 对顶栈，两个栈互相倒
struct TextEditor3 {
    left: Vec<u8>,
    right: Vec<u8>,
}


impl TextEditor3 {
    fn new() -> Self {
        Self { left: vec![], right: vec![] }
    }

    fn add_text(&mut self, text: String) {
        for &ch in text.as_bytes() {
            self.left.push(ch);
        }
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k.min(self.left.len() as i32);
        for _ in 0..k {
            self.left.pop();
        }
        k
    }

    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 && !self.left.is_empty() {
            self.right.push(self.left.pop().unwrap());
            k -= 1;
        }
        unsafe { String::from_utf8_unchecked(self.left[(self.left.len() as i32 - 10).max(0) as usize..].to_vec()) }
    }

    fn cursor_right(&mut self, mut k: i32) -> String {
        while k > 0 && !self.right.is_empty() {
            self.left.push(self.right.pop().unwrap());
            k -= 1;
        }
        unsafe { String::from_utf8_unchecked(self.left[(self.left.len() as i32 - 10).max(0) as usize..].to_vec()) }
    }
}

fn main() {
    let mut te = TextEditor::new();
    te.add_text(String::from("leetcode"));
    assert_eq!(te.delete_text(4), 4);
    te.add_text(String::from("practice"));
    assert_eq!(te.cursor_right(3), "etpractice");
    assert_eq!(te.cursor_left(8), "leet");
    assert_eq!(te.delete_text(10), 4);
    assert_eq!(te.cursor_left(2), "");
    assert_eq!(te.cursor_right(6), "practi");

    let mut te = TextEditor2::new();
    te.print();
    te.add_text(String::from("leetcode"));
    te.print();
    assert_eq!(te.delete_text(4), 4);
    te.print();
    te.add_text(String::from("practice"));
    te.print();
    assert_eq!(te.cursor_right(3), "etpractice");
    te.print();
    assert_eq!(te.cursor_left(8), "leet");
    te.print();
    assert_eq!(te.delete_text(10), 4);
    te.print();
    assert_eq!(te.cursor_left(2), "");
    te.print();
    assert_eq!(te.cursor_right(6), "practi");
    te.print();

    let mut te = TextEditor3::new();
    te.add_text(String::from("leetcode"));
    assert_eq!(te.delete_text(4), 4);
    te.add_text(String::from("practice"));
    assert_eq!(te.cursor_right(3), "etpractice");
    assert_eq!(te.cursor_left(8), "leet");
    assert_eq!(te.delete_text(10), 4);
    assert_eq!(te.cursor_left(2), "");
    assert_eq!(te.cursor_right(6), "practi");
}
