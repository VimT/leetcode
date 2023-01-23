//! LFU 缓存

use std::collections::HashMap;
use std::ptr::null_mut;

struct Node {
    prev: *mut Node,
    next: *mut Node,
    freq: i32,
    key: i32,
    value: i32,
}

impl Node {
    unsafe fn new(freq: i32, key: i32, value: i32) -> *mut Node {
        let node = Box::new(Node { prev: null_mut(), next: null_mut(), freq, key, value });
        let p = Box::into_raw(node);
        println!("alloc: {:p}, {}, {}, {}", p, freq, key, value);
        return p;
    }

    unsafe fn new_head() -> *mut Node {
        let node = Node::new(0, 0, 0);
        (*node).prev = node;
        (*node).next = node;
        node
    }

    unsafe fn is_empty(node: *mut Node) -> bool {
        (*node).next == node
    }
}

unsafe fn insert(node: *mut Node, new: *mut Node) {
    let nxt = (*node).next;
    (*node).next = new;
    (*new).prev = node;
    (*new).next = nxt;
    (*nxt).prev = new;
}

unsafe fn delete(node: *mut Node) {
    let prev = (*node).prev;
    let next = (*node).next;
    (*prev).next = next;
    (*next).prev = prev;
}

struct LFUCache {
    m: HashMap<i32, *mut Node>,
    freq_table: HashMap<i32, *mut Node>,
    cap: usize,
    min_freq: i32,
}

impl Drop for LFUCache {
    fn drop(&mut self) {
        unsafe {
            let keys: Vec<i32> = self.freq_table.keys().map(|x| *x).collect();
            for freq in keys {
                self.remove_freq(freq);
            }
        }
    }
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        Self { m: HashMap::with_capacity(cap + 1), freq_table: HashMap::with_capacity(cap + 1), cap, min_freq: 1 }
    }

    unsafe fn remove_freq(&mut self, freq: i32) {
        if let Some(head) = self.freq_table.remove(&freq) {
            let mut p = head;
            loop {
                let nxt = (*p).next;
                delete(p);
                let _ = Box::from_raw(p);
                println!("free: {:p}", p);
                if nxt == p { break; }
                p = nxt;
            }
        }
    }

    unsafe fn inc_freq(&mut self, node: *mut Node) {
        let freq = (*node).freq;
        delete(node);
        let head = *self.freq_table.entry(freq + 1).or_insert(Node::new_head());
        (*node).freq += 1;
        insert(head, node);
        if let Some(&head) = self.freq_table.get(&freq) {
            if Node::is_empty(head) {
                self.remove_freq(freq);
                if self.min_freq == freq {
                    self.min_freq += 1;
                }
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        unsafe {
            if let Some(&node) = self.m.get(&key) {
                self.inc_freq(node);
                return (*node).value;
            }
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 { return; }
        unsafe {
            if let Some(&node) = self.m.get(&key) {
                (*node).value = value;
                self.inc_freq(node);
            } else {
                let mut node = null_mut();
                if self.m.len() == self.cap {
                    let head = *self.freq_table.get(&self.min_freq).unwrap();
                    let will_remove = (*head).prev;
                    let key = (*will_remove).key;
                    self.m.remove(&key);
                    delete(will_remove);
                    if Node::is_empty(head) {
                        self.remove_freq(self.min_freq);
                    }
                    node = will_remove;
                }
                if node.is_null() {
                    node = Node::new(1, key, value);
                } else {
                    (*node).freq = 1;
                    (*node).key = key;
                    (*node).value = value;
                }
                self.m.insert(key, node);
                let head = *self.freq_table.entry(1).or_insert(Node::new_head());
                insert(head, node);
                self.min_freq = 1;
            }
        }
    }
}


fn main() {
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
    lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
    assert_eq!(lfu.get(1), 1);      // 返回 1
    // cache=[1,2], cnt(2)=1, cnt(1)=2
    lfu.put(3, 3);   // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
    // cache=[3,1], cnt(3)=1, cnt(1)=2
    assert_eq!(lfu.get(2), -1);      // 返回 -1（未找到）
    assert_eq!(lfu.get(3), 3);      // 返回 3
    // cache=[3,1], cnt(3)=2, cnt(1)=2
    lfu.put(4, 4);   // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
    // cache=[4,3], cnt(4)=1, cnt(3)=2
    assert_eq!(lfu.get(1), -1);      // 返回 -1（未找到）
    assert_eq!(lfu.get(3), 3);      // 返回 3
    // cache=[3,4], cnt(4)=1, cnt(3)=3
    assert_eq!(lfu.get(4), 4);      // 返回 4
    // cache=[3,4], cnt(4)=2, cnt(3)=3

    let mut lfu = LFUCache::new(0);
    lfu.put(0, 0);
    assert_eq!(lfu.get(0), -1);

    let mut lfu = LFUCache::new(10);
    lfu.put(10, 13);
    lfu.put(3, 17);
    lfu.put(6, 11);
    lfu.put(10, 5);
    lfu.put(9, 10);
    assert_eq!(lfu.get(13), -1);
    lfu.put(2, 19);
    assert_eq!(lfu.get(2), 19);
    assert_eq!(lfu.get(3), 17);
    lfu.put(5, 25);
    assert_eq!(lfu.get(8), -1);
    lfu.put(9, 22);
    lfu.put(5, 5);
    lfu.put(1, 30);
    assert_eq!(lfu.get(11), -1);
    lfu.put(9, 12);
    assert_eq!(lfu.get(7), -1);
    assert_eq!(lfu.get(5), 5);
    assert_eq!(lfu.get(8), -1);
    assert_eq!(lfu.get(9), 12);
    lfu.put(4, 30);
    lfu.put(9, 3);
    assert_eq!(lfu.get(9), 3);
    assert_eq!(lfu.get(10), 5);
    assert_eq!(lfu.get(10), 5);
    lfu.put(6, 14);
    lfu.put(3, 1);
    assert_eq!(lfu.get(3), 1);
    lfu.put(10, 11);
    assert_eq!(lfu.get(8), -1);
    lfu.put(2, 14);
    assert_eq!(lfu.get(1), 30);
    assert_eq!(lfu.get(5), 5);
    assert_eq!(lfu.get(4), 30);
    lfu.put(11, 4);
    lfu.put(12, 24);
    lfu.put(5, 18);
    assert_eq!(lfu.get(13), -1);
    lfu.put(7, 23);
    assert_eq!(lfu.get(8), -1);
    assert_eq!(lfu.get(12), 24);
    lfu.put(3, 27);
    lfu.put(2, 12);
    assert_eq!(lfu.get(5), 18);
    lfu.put(2, 9);
    lfu.put(13, 4);
    lfu.put(8, 18);
    lfu.put(1, 7);
    assert_eq!(lfu.get(6), 14);
    lfu.put(9, 29);
    lfu.put(8, 21);
    assert_eq!(lfu.get(5), 18);
    lfu.put(6, 30);
    lfu.put(1, 12);
    assert_eq!(lfu.get(10), 11);
    lfu.put(4, 15);
    lfu.put(7, 22);
    lfu.put(11, 26);
    lfu.put(8, 17);
    lfu.put(9, 29);
    assert_eq!(lfu.get(5), 18);
    lfu.put(3, 4);
    lfu.put(11, 30);
    assert_eq!(lfu.get(12), -1);
    lfu.put(4, 29);
    assert_eq!(lfu.get(3), 4);
    assert_eq!(lfu.get(9), 29);
    assert_eq!(lfu.get(6), 30);
    lfu.put(3, 4);
    assert_eq!(lfu.get(1), 12);
    assert_eq!(lfu.get(10), 11);
    lfu.put(3, 29);
    lfu.put(10, 28);
    lfu.put(1, 20);
    lfu.put(11, 13);
    assert_eq!(lfu.get(3), 29);
    lfu.put(3, 12);
    lfu.put(3, 8);
    lfu.put(10, 9);
    lfu.put(3, 26);
    assert_eq!(lfu.get(8), 17);
    assert_eq!(lfu.get(7), -1);
    assert_eq!(lfu.get(5), 18);
    lfu.put(13, 17);
    lfu.put(2, 27);
    lfu.put(11, 15);
    assert_eq!(lfu.get(12), -1);
    lfu.put(9, 19);
    lfu.put(2, 15);
    lfu.put(3, 16);
    assert_eq!(lfu.get(1), 20);
    lfu.put(12, 17);
    lfu.put(9, 1);
    lfu.put(6, 19);
    assert_eq!(lfu.get(4), 29);
    assert_eq!(lfu.get(5), 18);
    assert_eq!(lfu.get(5), 18);
    lfu.put(8, 1);
    lfu.put(11, 7);
    lfu.put(5, 2);
    lfu.put(9, 28);
    assert_eq!(lfu.get(1), 20);
    lfu.put(2, 2);
    lfu.put(7, 4);
    lfu.put(4, 22);
    lfu.put(7, 24);
    lfu.put(9, 26);
    lfu.put(13, 28);
    lfu.put(11, 26);
}

