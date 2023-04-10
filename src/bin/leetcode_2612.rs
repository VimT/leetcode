//! 最少翻转操作数

use std::collections::{BTreeSet, VecDeque};

pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let n = n as usize;
    let p = p as usize;
    let k = k as usize;
    let mut result = vec![-1; n];
    result[p] = 0;

    let mut set = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..n {
        set[i & 1].insert(i);
    }
    set[p & 1].remove(&p);
    for ban in banned {
        let ban = ban as usize;
        set[ban & 1].remove(&ban);
    }
    let mut q = VecDeque::with_capacity(n);
    q.push_back(p);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        let i = (u as i32 - k as i32 + 1).abs() as usize;
        let mut j = set[i & 1].range(i..).next().cloned();
        while j.is_some() && j.unwrap() < n - (n as i32 - u as i32 - k as i32).abs() as usize {
            let v = j.unwrap();
            q.push_back(v);
            result[v] = result[u] + 1;
            set[i & 1].remove(&v);
            j = set[i & 1].range(v..).next().cloned();
        }
    }
    result
}

pub fn min_reverse_operations2(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = vec![-1; n as usize];
    let mut sets: [BTreeSet<i32>; 2] =
        [(0..n).step_by(2).collect(), (1..n).step_by(2).collect()];
    for b in banned {
        sets[b as usize % 2].remove(&b);
    }
    sets[p as usize % 2].remove(&p);
    sets[0].insert(n);
    sets[1].insert(n);
    let mut wait = vec![p];
    let mut c = 0;
    while !wait.is_empty() {
        let mut nw = vec![];
        for i in wait.drain(..) {
            ans[i as usize] = c;
            let mn = (i + 1 - k).max(k - i - 1);
            let mx = (i + k - 1).min(n + n - k - i - 1);
            let s = &mut sets[mn as usize % 2];
            let v: Vec<_> = s.range(mn..=mx).copied().collect();
            for j in v {
                s.remove(&j);
                nw.push(j);
            }
        }
        c += 1;
        wait = nw;
    }
    ans
}

fn main() {
    fn test(func: fn(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32>) {
        assert_eq!(func(4, 3, vec![], 2), vec![3, 2, 1, 0]);
        assert_eq!(func(4, 2, vec![], 2), vec![2, 1, 0, 1]);
        assert_eq!(func(4, 2, vec![], 4), vec![-1, 1, 0, -1]);
        assert_eq!(func(2, 1, vec![], 2), vec![1, 0]);
        assert_eq!(func(4, 0, vec![1, 2], 4), vec![0, -1, -1, 1]);
        assert_eq!(func(5, 0, vec![2, 4], 3), vec![0, -1, -1, -1, -1]);
        assert_eq!(func(4, 2, vec![0, 1, 3], 1), vec![-1, -1, 0, -1]);
    }
    test(min_reverse_operations);
    test(min_reverse_operations2);
}
