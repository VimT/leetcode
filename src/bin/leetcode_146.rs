//! lru-cache

use std::collections::HashMap;
use std::ptr;

struct Node<T> {
    key: T,
    value: T,
    prev: NodePtr<T>,
    next: NodePtr<T>,
}

type BoxNode<T> = Box<Node<T>>;
type NodePtr<T> = *mut Node<T>;

impl<T> Node<T> {
    fn new(key: T, value: T) -> Self {
        Node {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

struct LRUCache {
    cache: HashMap<i32, BoxNode<i32>>,
    cap: usize,
    head: NodePtr<i32>,
    tail: NodePtr<i32>,
}


impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Box::into_raw(Box::new(Node::new(0, 0)));
        let tail = Box::into_raw(Box::new(Node::new(0, 0)));
        unsafe {
            head.as_mut().unwrap().next = tail;
            tail.as_mut().unwrap().next = head;
        }

        LRUCache {
            cache: HashMap::with_capacity(capacity as usize),
            cap: capacity as usize,
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get_mut(&key) {
            Some(node) => {
                let p: NodePtr<i32> = &mut **node;
                self.detach(p);
                self.attach(p);
                unsafe { p.as_ref() }.unwrap().value
            }
            None => -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let p = self.cache.get_mut(&key).map(|node| &mut **node as NodePtr<i32>);
        match p {
            Some(p) => {
                unsafe { p.as_mut().unwrap().value = value; }
                self.detach(p);
                self.attach(p);
            }
            None => {
                let mut node = if self.cache.len() == self.cap {
                    let old_key = unsafe { self.tail.as_ref().unwrap().prev.as_ref() }.unwrap().key;
                    let mut old_node = self.cache.remove(&old_key).unwrap();
                    old_node.key = key;
                    old_node.value = value;
                    let p: NodePtr<i32> = &mut *old_node;
                    self.detach(p);
                    old_node
                } else {
                    Box::new(Node::new(key, value))
                };
                let p = &mut *node;
                self.attach(p);
                self.cache.insert(key, node);
            }
        }
    }

    fn detach(&self, node: NodePtr<i32>) {
        unsafe {
            node.as_mut().unwrap().prev.as_mut().unwrap().next = node.as_mut().unwrap().next;
            node.as_mut().unwrap().next.as_mut().unwrap().prev = node.as_mut().unwrap().prev;
        }
    }

    /// attach to head
    fn attach(&self, node: NodePtr<i32>) {
        unsafe {
            node.as_mut().unwrap().next = self.head.as_ref().unwrap().next;
            node.as_mut().unwrap().prev = self.head;
            self.head.as_mut().unwrap().next = node;
            node.as_mut().unwrap().next.as_mut().unwrap().prev = node;
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}