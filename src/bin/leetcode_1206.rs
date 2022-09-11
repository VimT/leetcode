//! 设计跳表

use std::ptr::NonNull;
use rand::random;

const MAX_LEVEL: usize = 32;
const P_FACTOR: f64 = 0.25;

fn random_level() -> usize {
    let mut lv = 1;
    while lv < MAX_LEVEL && random::<f64>() < P_FACTOR {
        lv += 1;
    }
    lv
}

struct SkipListNode {
    val: i32,
    next: Vec<Option<NonNull<SkipListNode>>>,
}

impl SkipListNode {
    fn new(val: i32, level: usize) -> Self {
        Self {
            val,
            next: vec![None; level],
        }
    }

    pub unsafe fn take_tail(&mut self) -> Option<Box<Self>> {
        self.next[0].take().map(|p| {
            Box::from_raw(p.as_ptr())
        })
    }
}

impl Drop for SkipListNode {
    fn drop(&mut self) {
        unsafe {
            let mut node = self.take_tail();
            while let Some(mut node_inner) = node {
                node = node_inner.take_tail();
            }
        }
    }
}

struct Skiplist {
    head: Box<SkipListNode>,
    level: usize,
}


impl Skiplist {
    fn new() -> Self {
        Self {
            head: Box::new(SkipListNode::new(0, MAX_LEVEL)),
            level: 0,
        }
    }

    fn search(&self, target: i32) -> bool {
        unsafe {
            let node = (0..self.level).rev().fold(&*self.head, |mut node, level| {
                while node.next[level].is_some() && node.next[level].unwrap().as_ref().val < target {
                    node = node.next[level].unwrap().as_ref();
                }
                node
            });
            node.next[0].map(|x| x.as_ref().val == target).unwrap_or(false)
        }
    }

    fn add(&mut self, num: i32) {
        unsafe {
            let lv = random_level();
            self.level = self.level.max(lv);
            let new_node = Box::new(SkipListNode::new(num, lv));
            let mut ptr = NonNull::new_unchecked(Box::into_raw(new_node));
            let mut cur = &mut *self.head;
            // 找<num的最大节点，设置节点的next为 new_node
            for i in (0..self.level).rev() {
                while cur.next[i].is_some() && cur.next[i].unwrap().as_ref().val < num {
                    cur = cur.next[i].unwrap().as_mut();
                }
                if i < lv {
                    ptr.as_mut().next[i] = cur.next[i];
                    cur.next[i] = Some(ptr);
                }
            }
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        unsafe {
            let mut prev = vec![&mut *self.head as *mut SkipListNode; MAX_LEVEL];
            let mut cur = &*self.head;
            for i in (0..self.level).rev() {
                while cur.next[i].is_some() && cur.next[i].unwrap().as_ref().val < num {
                    cur = cur.next[i].unwrap().as_ref();
                }
                prev[i] = cur as *const _ as *mut _;
            }
            if cur.next[0].is_none() || cur.next[0].unwrap().as_ref().val != num {
                return false;
            }
            let cur_ptr = cur.next[0].unwrap();
            cur = cur.next[0].unwrap().as_ref();
            // 从下往上删除
            for i in 0..self.level {
                if (*prev[i]).next[i] != Some(cur_ptr) {
                    break;
                }
                (*prev[i]).next[i] = cur.next[i];
            }
            while self.level > 1 && self.head.next[self.level - 1].is_none() {
                self.level -= 1;
            }
            true
        }
    }
}

mod safe {
    use std::rc::Rc;
    use std::cell::RefCell;
    use rand::{Rng, thread_rng};

    type ListNode = Rc<RefCell<Node>>;

    fn new_list_node(val: i32) -> ListNode {
        Rc::new(RefCell::new(Node::new(val)))
    }

    struct Node {
        val: i32,
        next: Option<ListNode>,
        down: Option<ListNode>,
    }

    impl Node {
        fn new(val: i32) -> Node {
            Node { val, next: None, down: None }
        }
    }

    pub(crate) struct Skiplist {
        head: ListNode,
    }

    impl Skiplist {
        pub(crate) fn new() -> Self {
            Skiplist { head: new_list_node(-1) }
        }

        pub(crate) fn search(&self, target: i32) -> bool {
            let mut curr = Some(self.head.clone());
            let mut queue = Vec::new();
            while let Some(node) = curr {
                curr = if node.borrow().next.clone().filter(|n| n.borrow().val <= target).is_some() { node.borrow().next.clone() } else {
                    queue.push(node.clone());
                    node.borrow().down.clone()
                };
            }
            queue.last().filter(|n| n.borrow().val == target).is_some()
        }

        pub(crate) fn add(&mut self, num: i32) {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            let mut curr = Some(self.head.clone());
            let mut is_insert = true;
            let mut down = None;

            while let Some(node) = curr {
                if node.borrow().next.clone().filter(|n| n.borrow().val <= num).is_some() { curr = node.borrow().next.clone(); } else {
                    queue.push_back(node.clone());
                    curr = node.borrow().down.clone();
                }
            }

            while is_insert && !queue.is_empty() {
                if let Some(curr) = queue.pop_back() {
                    let next = new_list_node(num);
                    next.borrow_mut().next = curr.borrow().next.clone();
                    next.borrow_mut().down = down;
                    curr.as_ref().borrow_mut().next = Some(next);
                    down = curr.borrow_mut().next.clone();
                    is_insert = thread_rng().gen_range(0.0f64, 1.0f64) < 0.2;
                } else { break; }
            }
            if is_insert {
                let node = new_list_node(-1);
                node.borrow_mut().down = Some(self.head.clone());
                self.head = node;
            }
        }

        pub(crate) fn erase(&mut self, num: i32) -> bool {
            let mut curr = Some(self.head.clone());
            let mut is_found = false;
            while let Some(node) = curr {
                curr = if node.borrow().next.clone().filter(|n| n.borrow().val < num).is_some() { node.borrow().next.clone() } else { node.borrow().down.clone() };
                if node.borrow().next.is_some() && node.borrow().next.as_ref().unwrap().borrow().val == num {
                    is_found = true;
                    let next = node.borrow_mut().next.as_ref().unwrap().borrow_mut().next.clone();
                    node.borrow_mut().next = next;
                }
            }
            is_found
        }
    }
}

fn main() {
    let mut skiplist = Skiplist::new();
    skiplist.add(1);
    skiplist.add(2);
    skiplist.add(3);
    assert_eq!(skiplist.search(0), false);   // 返回 false
    skiplist.add(4);
    assert_eq!(skiplist.search(1), true);   // 返回 true
    assert_eq!(skiplist.erase(0), false);    // 返回 false，0 不在跳表中
    assert_eq!(skiplist.erase(1), true);    // 返回 true
    assert_eq!(skiplist.search(1), false);   // 返回 false，1 已被擦除

    let mut sl = safe::Skiplist::new();
    sl.add(0);
    sl.add(5);
    sl.add(2);
    sl.add(1);
    assert_eq!(sl.search(0), true);
    assert_eq!(sl.erase(5), true);
    assert_eq!(sl.search(2), true);
    assert_eq!(sl.search(3), false);
    assert_eq!(sl.search(2), true);
}
