//! 打开转盘锁

use std::collections::{BinaryHeap, VecDeque};

use leetcode::svec;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    if target == "0000" { return 0; }
    let mut dead = vec![false; 10001];
    for d in deadends {
        dead[d.parse::<usize>().unwrap()] = true;
    }
    if dead[0] { return -1; }
    let target: usize = target.parse().unwrap();
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    fn next(x: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(8);
        for i in 1..=4 {
            let base = 10_usize.pow(i);
            let wei_num = x / (base / 10) % 10;
            let small = (wei_num + 9) % 10;
            let big = (wei_num + 11) % 10;
            result.push(x / base * base + small * (base / 10) + x % (base / 10));
            result.push(x / base * base + big * (base / 10) + x % (base / 10));
        }
        result
    }
    let mut seen = vec![false; 10001];
    while !q.is_empty() {
        let (cur, cnt) = q.pop_front().unwrap();
        if cur == target { return cnt; }
        for nxt in next(cur) {
            if !seen[nxt] && !dead[nxt] {
                q.push_back((nxt, cnt + 1));
                seen[nxt] = true;
            }
        }
    }
    -1
}

pub fn open_lock_astar(deadends: Vec<String>, target: String) -> i32 {
    if target == "0000" { return 0; }
    let mut dead = vec![false; 10001];
    for d in deadends {
        dead[d.parse::<usize>().unwrap()] = true;
    }
    if dead[0] { return -1; }
    let target: usize = target.parse().unwrap();
    fn next(x: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(8);
        for i in 1..=4 {
            let base = 10_usize.pow(i);
            let wei_num = x / (base / 10) % 10;
            let small = (wei_num + 9) % 10;
            let big = (wei_num + 11) % 10;
            result.push(x / base * base + small * (base / 10) + x % (base / 10));
            result.push(x / base * base + big * (base / 10) + x % (base / 10));
        }
        result
    }
    let mut seen = vec![false; 10001];

    let mut q = BinaryHeap::new();
    #[inline]
    fn dis(mut x: usize, mut target: usize) -> i32 {
        let mut result = 0;
        for _ in 0..4 {
            result += ((x % 10 + 10 - target % 10) % 10).min((target % 10 + 10 - x % 10) % 10) as i32;
            x /= 10;
            target /= 10;
        }
        result
    }
    q.push((-dis(0, target), 0, 0));
    seen[0] = true;

    while !q.is_empty() {
        let (_, cnt, cur) = q.pop().unwrap();
        if cur == target { return cnt; }
        for nxt in next(cur) {
            if !seen[nxt] && !dead[nxt] {
                q.push((-(cnt + 1 + dis(nxt, target)), cnt + 1, nxt));
                seen[nxt] = true;
            }
        }
    }
    -1
}


fn main() {
    assert_eq!(open_lock_astar(svec!["0201", "0101", "0102", "1212", "2002"], String::from("0202")), 6);
    assert_eq!(open_lock_astar(svec!["8888"], String::from("0009")), 1);
    assert_eq!(open_lock_astar(svec!["0000"], String::from("8888")), -1);
    assert_eq!(open_lock_astar(svec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"], String::from("8888")), -1);
}
