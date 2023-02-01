//! 你可以安排的最多任务数目

use std::collections::BTreeMap;
use leetcode::union_set::UnionSet;

// 520ms, n(logn)2
pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    tasks.sort_unstable_by_key(|x| -x);
    workers.sort_unstable_by_key(|x| -x);
    let mut left = 0;
    let mut right = tasks.len().min(workers.len());
    fn ok(workers: &[i32], tasks: &[i32], mut p: i32, s: i32) -> bool {
        let mut m: BTreeMap<i32, i32> = BTreeMap::new();
        for &w in workers {
            *m.entry(w).or_default() += 1;
        }
        for &t in tasks {
            if m.is_empty() { return false; }
            let cur = *m.range(..).last().unwrap().0;
            if cur >= t {
                *m.get_mut(&cur).unwrap() -= 1;
                if m[&cur] == 0 {
                    m.remove(&cur);
                }
            } else {
                if p == 0 { return false; }
                let w = m.range(t - s..).next();
                if w.is_none() {
                    return false;
                }
                let w = *w.unwrap().0;
                p -= 1;
                *m.get_mut(&w).unwrap() -= 1;
                if m[&w] == 0 { m.remove(&w); }
            }
        }
        true
    }
    while left < right {
        let mid = (left + right + 1) / 2;
        if ok(&workers[..mid], &tasks[tasks.len() - mid..], pills, strength) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}


// 52ms, nlogn
pub fn max_task_assign_union_set(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    tasks.sort_unstable();
    workers.sort_unstable();
    let mut left = 0;
    let mut right = tasks.len().min(workers.len());
    let mut to = vec![0; workers.len()];
    for i in 0..workers.len() {
        // workers[i] 吃药之后能完成的最大任务
        to[i] = tasks.binary_search(&(workers[i] + strength)).unwrap_or_else(|x| x - 1);
    }
    fn ok(workers: &Vec<i32>, tasks: &Vec<i32>, to: &Vec<usize>, mut p: i32, mid: usize) -> bool {
        // tasks[0..mid-1], workers[-c..]
        // 并查集： 作用1：标记某个tasks是否已用过，作用2：find(i) 找<=i的最大任务
        let mut us = UnionSet::new(mid + 1);
        let mut i = 0;
        for j in workers.len() - mid..workers.len() {
            while us.find(i + 1) != i + 1 { i += 1; }
            if tasks[i] <= workers[j] {
                // 不吃药能完成
                us.union_force(i, i + 1);
                continue;
            }
            // 要吃药
            if p == 0 { return false; }
            p -= 1;
            // 吃药完成的是 to[j] 或者是最后一个任务
            let z = us.find(to[j].min(mid - 1) + 1);
            if z == 0 { return false; }
            us.union_force(z - 1, z);
        }
        true
    }
    while left < right {
        let mid = (left + right + 1) / 2;
        if ok(&workers, &tasks, &to, pills, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn main() {
    assert_eq!(max_task_assign_union_set(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10), 2);
    assert_eq!(max_task_assign_union_set(vec![5, 9, 8, 5, 9], vec![1, 6, 4, 2, 6], 1, 5), 3);
    assert_eq!(max_task_assign_union_set(vec![35], vec![83, 20, 4, 66], 3, 41), 1);
    assert_eq!(max_task_assign_union_set(vec![3, 2, 1], vec![0, 3, 3], 1, 1), 3);
    assert_eq!(max_task_assign_union_set(vec![5, 4], vec![0, 0, 0], 1, 5), 1);
}
