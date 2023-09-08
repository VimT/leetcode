//! 找到处理最多请求的服务器

use std::collections::{BinaryHeap, BTreeSet};

pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let mut free: BTreeSet<i32> = (0..k).collect();
    let mut doing: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut server_tasks = vec![0; k as usize];
    for ((start, load), i) in arrival.into_iter().zip(load).zip(0..) {
        while !doing.is_empty() && -doing.peek().unwrap().0 <= start {
            let (_, server) = doing.pop().unwrap();
            free.insert(server);
        }
        let want = i % k;
        if let Some(&server) = free.range(want..).next() {
            free.remove(&server);
            doing.push((-start - load, server));
            server_tasks[server as usize] += 1;
            continue;
        }
        if let Some(&server) = free.range(..want).next() {
            free.remove(&server);
            doing.push((-start - load, server));
            server_tasks[server as usize] += 1;
        }
    }
    let mx = *server_tasks.iter().max().unwrap();
    (0..k).filter(|&x| server_tasks[x as usize] == mx).collect()
}

/// 双优先队列
pub fn busiest_servers2(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let mut free: BinaryHeap<i32> = (0..k).map(|x| -x).collect();
    let mut doing: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut server_tasks = vec![0; k as usize];
    for ((start, load), i) in arrival.into_iter().zip(load).zip(0..) {
        while !doing.is_empty() && -doing.peek().unwrap().0 <= start {
            let (_, server) = doing.pop().unwrap();
            free.push(-(i + ((server - i) % k + k) % k));
        }
        if !free.is_empty() {
            let server = -free.pop().unwrap() % k;
            server_tasks[server as usize] += 1;
            doing.push((-start - load, server));
        }
    }
    let mx = *server_tasks.iter().max().unwrap();
    (0..k).filter(|&x| server_tasks[x as usize] == mx).collect()
}

fn main() {
    fn test(func: fn(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]), vec![1]);
        assert_eq!(func(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]), vec![0]);
        assert_eq!(func(3, vec![1, 2, 3], vec![10, 12, 11]), vec![0, 1, 2]);
        assert_eq!(func(3, vec![1, 2, 3, 4, 8, 9, 10], vec![5, 2, 10, 3, 1, 2, 2]), vec![1]);
        assert_eq!(func(1, vec![1], vec![1]), vec![0]);
    }
    test(busiest_servers);
    test(busiest_servers2);
}
