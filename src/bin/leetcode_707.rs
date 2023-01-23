//! 设计链表

use std::ptr::null_mut;

struct Node {
    prev: *mut Node,
    next: *mut Node,
    val: i32,
}

impl Node {
    fn new(val: i32) -> *mut Node {
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

struct MyLinkedList {
    head: *mut Node,
    len: i32,
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        unsafe {
            let mut node = self.head;
            for _ in 0..=self.len {
                let next = (*node).next;
                let _ = Box::from_raw(node);
                node = next;
            }
        }
    }
}


impl MyLinkedList {
    fn new() -> Self {
        unsafe {
            let head = Node::new(0);
            (*head).prev = head;
            (*head).next = head;
            Self { head, len: 0 }
        }
    }

    fn get(&self, index: i32) -> i32 {
        unsafe {
            if index >= self.len || index < 0 { return -1; }
            let mut node = self.head;
            for _ in 0..=index {
                node = (*node).next;
            }
            (*node).val
        }
    }

    fn add_at_head(&mut self, val: i32) {
        unsafe {
            self.len += 1;
            Node::insert(self.head, Node::new(val));
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        unsafe {
            self.len += 1;
            Node::insert((*self.head).prev, Node::new(val));
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        unsafe {
            if index > self.len { return; }
            if index <= 0 {
                self.add_at_head(val);
            } else if index == self.len {
                self.add_at_tail(val);
            } else {
                let mut node = self.head;
                for _ in 0..index {
                    node = (*node).next;
                }
                Node::insert(node, Node::new(val));
                self.len += 1;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        unsafe {
            if index >= self.len || index < 0 { return; }
            self.len -= 1;
            let mut node = self.head;
            for _ in 0..=index {
                node = (*node).next;
            }
            Node::remove(node);
            let _ = Box::from_raw(node);
        }
    }
}

mod other_weak_rc {
    use std::cell::{RefCell};
    use std::ops::Deref;
    use std::rc::{Rc, Weak};

    //双向链表：
    #[derive(Default)]
    struct MyLinkedList {
        size: i32,
        dummy_head: Rc<RefCell<Node>>,
        dummy_tail: Rc<RefCell<Node>>,
    }

    #[derive(Default)]
    struct Node {
        val: i32,
        pre: Option<Weak<RefCell<Node>>>,
        next: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new_rc_ref_node(val: i32) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                val,
                pre: None,
                next: None,
            }))
        }
    }

    impl MyLinkedList {
        fn connect(pre: &Rc<RefCell<Node>>, next: &Rc<RefCell<Node>>) {
            pre.deref().borrow_mut().next = Some(next.clone());
            next.deref().borrow_mut().pre = Some(Rc::downgrade(pre));
        }

        fn new() -> Self {
            let new_node = MyLinkedList {
                size: 0,
                dummy_head: Rc::new(RefCell::new(Node::default())),
                dummy_tail: Rc::new(RefCell::new(Node::default())),
            };
            MyLinkedList::connect(&new_node.dummy_head, &new_node.dummy_tail);
            new_node
        }

        /// index :[-1,size] ;
        /// if index==-1, return dummy_head's rc;
        /// if index==size, return dummy_tail's rc;
        /// else return node's rc;
        fn get_rc(&self, index: i32) -> Option<Rc<RefCell<Node>>> {
            if index >= -1 && index <= self.size {
                let mut now = self.dummy_head.clone();
                for _ in 0..=index {
                    let next = now.deref().borrow().next.as_ref().unwrap().clone();
                    now = next;
                }
                Some(now)
            } else {
                None
            }
        }
        fn get_next_rc_from_rc(rc_now: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
            rc_now.deref().borrow().next.clone()
        }
        fn add_at_index(&mut self, index: i32, val: i32) {
            if index >= 0 && index <= self.size {
                //now是index 的前一个。
                let now = self.get_rc(index - 1).unwrap(); //找到index的前一个：
                let old_nodes = (*now).borrow_mut().next.take().unwrap();
                let new_rc_node = Node::new_rc_ref_node(val);
                MyLinkedList::connect(&now, &new_rc_node);
                MyLinkedList::connect(&new_rc_node, &old_nodes);

                self.size += 1;
            }
        }

        fn add_at_head(&mut self, val: i32) {
            self.add_at_index(0, val);
        }
        fn add_at_tail(&mut self, val: i32) {
            self.add_at_index(self.size, val);
        }

        //get(index)：获取链表中第 index 个节点的值。如果索引无效，则返回-1。
        fn get(&self, index: i32) -> i32 {
            if index >= 0 && index < self.size {
                self.get_rc(index).unwrap().deref().borrow().val
            } else {
                -1
            }
        }
        fn delete_at_index(&mut self, index: i32) {
            if index >= 0 && index < self.size {
                let delet_pre = self.get_rc(index - 1).unwrap();
                let next = Self::get_next_rc_from_rc(&delet_pre).unwrap();
                let next_next = Self::get_next_rc_from_rc(&next);
                delet_pre.deref().borrow_mut().next = next_next;
                self.size -= 1;
            }
        }
    }

    pub fn test() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);   //链表变为1-> 2-> 3
        assert_eq!(list.get(1), 2);            //返回2
        list.delete_at_index(1);  //现在链表是1-> 3
        assert_eq!(list.get(1), 3);            //返回3
    }
}

fn main() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);   //链表变为1-> 2-> 3
    assert_eq!(list.get(1), 2);            //返回2
    list.delete_at_index(1);  //现在链表是1-> 3
    assert_eq!(list.get(1), 3);            //返回3

    list = MyLinkedList::new();
    list.add_at_tail(1);
    assert_eq!(list.get(0), 1);

    other_weak_rc::test();
}
