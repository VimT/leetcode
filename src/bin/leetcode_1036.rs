//! 逃离大迷宫

use std::collections::{BTreeSet, HashMap, VecDeque};

/// 离散化
pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    const MAX: i32 = 1e6 as i32;
    let mut x: BTreeSet<i32> = BTreeSet::new();
    let mut y: BTreeSet<i32> = BTreeSet::new();
    x.insert(source[0]);
    x.insert(target[0]);
    y.insert(source[1]);
    y.insert(target[1]);
    for block in &blocked {
        x.insert(block[0]);
        if block[0] > 0 { x.insert(block[0] - 1); };
        if block[0] + 1 < MAX { x.insert(block[0] + 1); };
        y.insert(block[1]);
        if block[1] > 0 { y.insert(block[1] - 1); };
        if block[1] + 1 < MAX { y.insert(block[1] + 1); };
    }
    let x: Vec<i32> = x.into_iter().collect();
    let y: Vec<i32> = y.into_iter().collect();
    let xm: HashMap<i32, usize> = x.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    let ym: HashMap<i32, usize> = y.iter().enumerate().map(|y| (*y.1, y.0)).collect();
    let mut q = VecDeque::new();
    let m = x.len();
    let n = y.len();
    let mut seen = vec![vec![false; n]; m];
    let mut map = vec![vec![false; n]; m];
    for block in &blocked {
        map[xm[&block[0]]][ym[&block[1]]] = true;
    }
    let source = (xm[&source[0]], ym[&source[1]]);
    q.push_back(source);
    seen[source.0][source.1] = true;
    let target = (xm[&target[0]], ym[&target[1]]);
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if (x, y) == target {
            return true;
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !map[nx][ny] && !seen[nx][ny] {
                    seen[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    false
}

fn main() {
    assert_eq!(is_escape_possible(vec![vec![0, 4], vec![2, 4], vec![3, 1], vec![3, 3], vec![4, 0], vec![4, 2]], vec![2, 2], vec![7, 3]), true);
    assert_eq!(is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]), false);
    assert_eq!(is_escape_possible(vec![], vec![0, 0], vec![999999, 999999]), true);
}
