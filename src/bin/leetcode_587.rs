//! 安装栅栏

use std::collections::HashSet;

use leetcode::unorder;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

/// Jarvis
pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn orientation(p: Point, q: Point, r: Point) -> i32 {
        (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
    }
    fn is_between(p: Point, i: Point, q: Point) -> bool {
        let a = i.x >= p.x && i.x <= q.x || i.x <= p.x && i.x >= q.x;
        let b = i.y >= p.y && i.y <= q.y || i.y <= p.y && i.y >= q.y;
        a && b
    }
    let trees: Vec<Point> = trees.into_iter().map(|x| Point { x: x[0], y: x[1] }).collect();
    let len = trees.len();
    if len < 4 {
        return trees.into_iter().collect::<HashSet<Point>>().into_iter().map(|x| vec![x.x, x.y]).collect();
    }
    let mut left_most = 0;
    for i in 0..len {
        if trees[i].x < trees[left_most].x {
            left_most = i;
        }
    }
    let mut hull = HashSet::new();
    let mut p = left_most;
    loop {
        let mut q = (p + 1) % len;
        for i in 0..len {
            if orientation(trees[p], trees[i], trees[q]) < 0 {
                q = i;
            }
        }
        for i in 0..len {
            if i != p && i != q && orientation(trees[p], trees[i], trees[q]) == 0 && is_between(trees[p], trees[i], trees[q]) {
                hull.insert(trees[i]);
            }
        }
        hull.insert(trees[q]);
        p = q;
        if p == left_most { break; }
    }
    hull.into_iter().map(|x| vec![x.x, x.y]).collect()
}

fn main() {
    assert_eq!(unorder(outer_trees(vec![vec![1, 1], vec![2, 2], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]])), unorder(vec![vec![1, 1], vec![2, 0], vec![3, 3], vec![2, 4], vec![4, 2]]));
    assert_eq!(unorder(outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]])), unorder(vec![vec![4, 2], vec![2, 2], vec![1, 2]]));
}
