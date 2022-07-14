//! 杀掉进程

use std::collections::{HashMap, VecDeque};

pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
    let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
    for (p, parent) in pid.into_iter().zip(ppid) {
        children.entry(parent).or_default().push(p);
    }
    let mut q = VecDeque::new();
    q.push_back(kill);
    let mut result = vec![];
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        result.push(node);
        if let Some(children) = children.get(&node) {
            for &child in children {
                q.push_back(child);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5), vec![5, 10]);
        assert_eq!(func(vec![1], vec![0], 1), vec![1]);
    }
    test(kill_process);
}
