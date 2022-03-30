//! 全 O(1) 的数据结构

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::ptr::null_mut;

struct Node {
    count: i32,
    keys: HashSet<String>,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    unsafe fn new(count: i32) -> *mut Node {
        Box::into_raw(Box::new(Node { count, keys: HashSet::new(), prev: null_mut(), next: null_mut() }))
    }
}

unsafe fn insert(cur: *mut Node, new: *mut Node) {
    let next = (*cur).next;
    (*new).prev = cur;
    (*cur).next = new;
    (*new).next = next;
    (*next).prev = new;
}

unsafe fn delete(node: *mut Node) {
    let prev = (*node).prev;
    let next = (*node).next;
    (*prev).next = next;
    (*next).prev = prev;
    let _ = Box::from_raw(node); // drop
}

struct AllOne {
    m: HashMap<String, *mut Node>,
    head: *mut Node,
}

impl Drop for AllOne {
    fn drop(&mut self) {
        unsafe {
            let mut node = self.head;
            while !node.is_null() {
                let next = (*node).next;
                delete(node);
                if next == node {
                    return;
                }
                node = next;
            }
        }
    }
}


impl AllOne {
    fn new() -> Self {
        unsafe {
            let node = Node::new(0);
            (*node).next = node;
            (*node).prev = node;
            AllOne {
                m: HashMap::new(),
                head: node,
            }
        }
    }

    fn inc(&mut self, key: String) {
        unsafe {
            match self.m.entry(key.clone()) {
                Entry::Occupied(e) => {
                    let old_node = *e.get();
                    let old_count = (*old_node).count;
                    if (*(*old_node).next).count != old_count + 1 {
                        insert(old_node, Node::new(old_count + 1))
                    }
                    let new_node = (*old_node).next;
                    (*new_node).keys.insert(key.clone());
                    self.m.insert(key.clone(), new_node);
                    (*old_node).keys.remove(&key);
                    if (*old_node).keys.is_empty() {
                        delete(old_node);
                    }
                }
                Entry::Vacant(e) => {
                    if (*(*self.head).next).count != 1 {
                        insert(self.head, Node::new(1));
                    }
                    let new_node = (*self.head).next;
                    (*new_node).keys.insert(key.clone());
                    e.insert(new_node);
                }
            }
        }
    }

    fn dec(&mut self, key: String) {
        unsafe {
            if let Entry::Occupied(e) = self.m.entry(key.clone()) {
                let old_node = *e.get();
                let old_count = (*old_node).count;
                if old_count == 1 {
                    e.remove();
                } else {
                    if (*(*old_node).prev).count != old_count - 1 {
                        insert((*old_node).prev, Node::new(old_count - 1));
                    }
                    let new_node = (*old_node).prev;
                    (*new_node).keys.insert(key.clone());
                    self.m.insert(key.clone(), new_node);
                }
                (*old_node).keys.remove(&key);
                if (*old_node).keys.is_empty() {
                    delete(old_node);
                }
            }
        }
    }

    fn get_max_key(&self) -> String {
        unsafe { (*(*self.head).prev).keys.iter().next().map(|x| x.clone()).unwrap_or(String::new()) }
    }

    fn get_min_key(&self) -> String {
        unsafe { (*(*self.head).next).keys.iter().next().map(|x| x.clone()).unwrap_or(String::new()) }
    }
}


fn main() {
    let mut obj = AllOne::new();
    obj.inc(String::from("count2"));
    obj.inc(String::from("count2"));
    obj.inc(String::from("count1"));
    println!("{}", obj.get_max_key());
    println!("{}", obj.get_min_key());
    obj.dec(String::from("key"));
    println!("{}", obj.get_max_key());
    println!("{}", obj.get_min_key());
}
