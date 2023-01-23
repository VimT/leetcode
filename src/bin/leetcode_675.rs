//! 为高尔夫比赛砍树

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn astar(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let m = forest.len();
    let n = forest[0].len();
    heap.push(Reverse((0, 0, sx, sy)));
    let mut cost = HashMap::new();
    cost.insert((sx, sy), 0);
    while !heap.is_empty() {
        let Reverse((_, g, x, y)) = heap.pop().unwrap();
        if x == tx && y == ty {
            return g;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && forest[nx as usize][ny as usize] > 0 {
                // h 是从 (nx, ny) 到 (tx, ty) 的距离的启发式
                let h = (nx - tx).abs() + (ny - ty).abs();
                // g 是从 (sx, sy) 到 (nx, xy) 的实际距离, h
                let next_cost = g + 1 + h;
                if next_cost < *cost.get(&(nx, ny)).unwrap_or(&9999) {
                    cost.insert((nx, ny), next_cost);
                    heap.push(Reverse((next_cost, g + 1, nx, ny)));
                }
            }
        }
    }
    -1
}

fn hadlocks(forest: &Vec<Vec<i32>>, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    let m = forest.len();
    let n = forest[0].len();
    let mut processed = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, sx, sy));
    while !q.is_empty() {
        // 称 detour 为一次绕开目标的移动
        let (detours, x, y) = q.pop_front().unwrap();
        if !processed.contains(&(x, y)) {
            processed.insert((x, y));
            if x == tx && y == ty { return (sx - tx).abs() + (sy - ty).abs() + 2 * detours; }
            for (nx, ny, closer) in [(x - 1, y, x > tx), (x + 1, y, x < tx), (x, y - 1, y > ty), (x, y + 1, y < ty)] {
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && forest[nx as usize][ny as usize] > 0 {
                    if closer {
                        q.push_front((detours, nx, ny));
                    } else {
                        q.push_back((detours + 1, nx, ny));
                    }
                }
            }
        }
    }
    -1
}

pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let mut trees = vec![];
    let m = forest.len();
    let n = forest[0].len();
    for i in 0..m {
        for j in 0..n {
            if forest[i][j] > 1 {
                trees.push((forest[i][j], i as i32, j as i32));
            }
        }
    }
    trees.sort_unstable();

    let (mut x, mut y) = (0, 0);
    let mut result = 0;
    for (_, nx, ny) in trees {
        let d = hadlocks(&forest, x, y, nx, ny);
        if d < 0 { return -1; }
        result += d;
        x = nx;
        y = ny;
    }
    result
}

fn main() {
    assert_eq!(cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]), 6);
    assert_eq!(cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]), -1);
    assert_eq!(cut_off_tree(vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6]]), 6);
}
